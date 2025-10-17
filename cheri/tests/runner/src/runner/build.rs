//! Command helpers.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use colored::Colorize;

use crate::runner::App;
use crate::runner::log::*;

impl App {
    /// Builds `libcheriot.rlib` and returns its location as a [`PathBuf`] relative to the given
    /// `build_artefacts_dir`.
    pub(crate) fn build_libcheriot(&self, build_artefacts_dir: &Path) -> anyhow::Result<PathBuf> {
        // `libcheriot` is the helper library that contains the definitions for the CHERIoT allocator, panic handler and more.
        // We need to build it here so we can manually link it with the tests later.
        let libcheriot_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/libcheriot"));

        let lib_path = libcheriot_path.join("src/lib.rs");
        let output_path = build_artefacts_dir.join("libcheriot.rlib");
        let output_dir = format!("--out-dir={}", build_artefacts_dir.display());

        self.rustc(&lib_path, &[&output_dir, "--crate-name=cheriot", "--crate-type=rlib"])?;
        self.rustc(&lib_path, &[&output_dir, "--crate-name=cheriot", "--crate-type=staticlib"])?;

        assert!(output_path.exists());
        Ok(output_path)
    }

    /// Compiles the `.rs` file pointed by `test_rs` into an object file, placing the result in
    /// `build_artefacts_dir/<test_name>/libtest.a`. It returns the path to the compiled object file.
    pub(crate) fn build_test(
        &self,
        build_artefacts_dir: &Path,
        test_rs: &Path,
    ) -> anyhow::Result<(PathBuf, String)> {
        // We have the absolute path to the directory that will contain the build artefacts,
        // we have the path to the test file relative to the cwd; we need to find the least common
        // ancestor between the test path and cwd to be able to recreate the same directory
        // structure in the build directory in order to avoid name clashes.

        let test_stem = test_rs
            .file_stem()
            .and_then(|v| v.to_str())
            .expect("test should have a valid file name!");

        let lca = |test_path: &Path| {
            let test_parent_path = test_path.parent().expect("test file should have a parent");
            assert!(test_parent_path.is_relative());

            // This is definitely not the right way to compute the LCA of test_path w.r.t. the cwd,
            // but we'll work with it for now.

            if test_parent_path.as_os_str() == "." || test_parent_path.as_os_str() == ".." {
                return String::new();
            }

            let mut res = String::from(
                test_parent_path
                    .file_name()
                    .expect("test file parent should have a valid file name")
                    .to_string_lossy(),
            );
            for ancestor in test_parent_path.ancestors() {
                let ancestor_str = ancestor.to_string_lossy();
                if ancestor_str == ".." || ancestor_str == "." {
                    break;
                }

                res = format!("{ancestor_str}/{res}");
            }
            res
        };

        let to_test_name = |test_path: &Path| {
            let mut res = lca(test_path);
            if !res.is_empty() {
                res.push('/');
            }
            res.push_str(test_stem);
            let res = res.replace(" ", "_");
            let res = res.replace("/", "_");
            res
        };

        let test_name = to_test_name(&test_rs);
        let output_path = build_artefacts_dir.join(&test_name);
        let output_dir = format!("--out-dir={}", output_path.display());
        let libcheriot_path = format!("-L{}", build_artefacts_dir.display());
        self.rustc(test_rs, &[&output_dir, &libcheriot_path, "--emit=llvm-ir"])?;
        self.rustc(
            test_rs,
            &[&output_dir, &libcheriot_path, "--extern=cheriot", "--crate-type=staticlib"],
        )?;
        let test_output_path = output_path.join(format!("lib{test_stem}.a"));
        assert!(
            test_output_path.exists(),
            "Not your fault, there's something wrong with how the runner resolves the paths of tests.. ({} does not exist when it should)",
            test_output_path.display()
        );

        std::fs::rename(&test_output_path, &output_path.join(format!("libtest.a")))?;
        Ok((output_path, test_name))
    }

    pub(crate) fn generate_runner(
        &self,
        test_name: &str,
        out_dir: &Path,
    ) -> anyhow::Result<PathBuf> {
        info!(self, "generating c++ shim...");

        let shim = format!(
            r#"#include "cheri.h"
#include "cheri.hh"
#include "token.h"
#include <cstdio>
#include <cstdlib>
#include <simulator.h>

extern "C" void __rust_test_panic(char *ptr) {{
  fprintf(stderr, "%s", ptr);
  free(ptr);
  simulation_exit(1);
}}

extern "C" void __rust_test_free(void *ptr) {{ free(ptr); }}

extern "C" void *__rust_test_alloc(int size) {{ return malloc(size); }}

extern "C" void __rust_test_print_str(char *str) {{ fprintf(stdout, "%s", str); }}
extern "C" void __rust_test_eprint_str(char *str) {{
  fprintf(stderr, "%s", str);
}}

unsigned short lfsr = 0xACE1u;
unsigned       bit;

extern "C" int __rust_test_rand_int()
{{
	bit         = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5)) & 1;
	return lfsr = (lfsr >> 1) | (bit << 15);
}}

extern "C" int {test_name}();

int __attribute__((cheriot_compartment("test_runner"))) run_tests() {{
    printf("@rust-test-runner-sync-start\n");
    int ret = {test_name}();
    printf("@rust-test-runner-sync-end\n");
    simulation_exit(ret);
}}"#
        );
        let shim_path = out_dir.join("runner.cc");

        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&shim_path)
            .map_err(|e| anyhow::anyhow!("failed to open {shim_path:?}: {e}"))?;
        file.write_all(shim.as_bytes())
            .map_err(|e| anyhow::anyhow!("failed to write to {shim_path:?}: {e}"))?;

        infoln!(self, "ok".bright_green());
        Ok(shim_path)
    }

    pub(crate) fn generate_xmake_config(
        &self,
        test_obj_path: &Path,
        test_name: &str,
        rtos_sdk_path: &Path,
        out_dir: &Path,
    ) -> anyhow::Result<()> {
        let xmake_config = format!(
            r#"
sdkdir = path.absolute("{}")

includes(sdkdir)

set_toolchains("cheriot-clang")

option("board")
  set_default("sail")

compartment("test_runner")
    add_deps("freestanding", "string", "crt", "cxxrt", "atomic_fixed", "compartment_helpers", "debug", "softfloat")
    add_deps("message_queue", "locks", "event_group", "cheriot.allocator")
    add_deps("stdio")
    add_deps("strtol")
	add_files("runner.cc")
	add_ldflags("-L{}", {{force = true}})
    add_ldflags("-l{test_name}", {{force = true}})

firmware("test")
    add_deps("test_runner")
    on_load(function(target)
        target:values_set("board", "$(board)")
        target:values_set("threads", {{
            {{
            compartment = "test_runner",
            priority = 1,
            entry_point = "run_tests",
            stack_size = 0x1F00,
            trusted_stack_frames = 6
          }},
        }}, {{expand = false}})
    end)
"#,
            rtos_sdk_path.display(),
            test_obj_path.display(),
        );

        info!(self, "generating xmake config...");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(out_dir.join("xmake.lua"))?;
        file.write(xmake_config.as_bytes())?;

        infoln!(self, "ok".bright_green());
        Ok(())
    }
}
