//! Command helpers.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use colored::Colorize;

use crate::runner::App;
use crate::runner::log::*;

impl App {
    /// Given the path to the test and the directory where every test will be compiled and run, retrieve
    /// the path and the unique name for this specific test.
    ///
    /// For example:
    ///     ("cheri-runner/objs", "my_test.rs") -> ("cheri-runner/objs/my_test", "my_test")
    ///     ("cheri-runner/objs", "specs/my_test.rs") -> ("cheri-runner/objs/specs/my_test", "specs_my_test")
    pub(crate) fn generate_test_env(
        &self,
        build_artefacts_dir: &Path,
        test_rs: &Path,
    ) -> anyhow::Result<(PathBuf, String, PathBuf)> {
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

            if test_parent_path.as_os_str().is_empty()
                || test_parent_path.as_os_str() == "."
                || test_parent_path.as_os_str() == ".."
            {
                return PathBuf::new();
            }

            test_parent_path
                .components()
                .rev()
                .map_while(|x| {
                    if matches!(x, Component::CurDir | Component::ParentDir) {
                        return None;
                    }

                    Some(x)
                })
                .collect::<PathBuf>()
        };

        let to_test_name = |test_path: &Path| {
            let mut res = lca(test_path);
            res.push(test_stem);
            let res = res.to_string_lossy();
            let res = res.replace(" ", "_");
            let res = res.replace("/", "_");
            res
        };

        let test_name = to_test_name(&test_rs);
        let test_dir_path = build_artefacts_dir.join(&test_name);
        std::fs::create_dir_all(&test_dir_path)?;
        let test_rs_path = test_dir_path.join(&test_name).with_extension("rs");

        std::fs::copy(test_rs, &test_rs_path)?;
        Ok((test_dir_path, test_name, test_rs_path))
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
#include <fail-simulator-on-error.h>

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
        test_rs_path: &Path,
        libcheriot_manifest_path: &Path,
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

target("libcheriot", function()
	set_plat("cheriot")
	set_arch("cheriot")
	add_files(
		"{}",
        {{ rules = {{ "cheriot.rust.crate", override = true }}, force = true, sourcekind = "cheriot.rust.crate" }}
	)
	on_run(function() end)
end)

compartment("test_runner")
    add_deps("freestanding", "string", "crt", "cxxrt", "atomic_fixed", "compartment_helpers", "debug", "softfloat")
    add_deps("message_queue", "locks", "event_group", "cheriot.allocator", "stdio", "strtol", "libcheriot")
	add_files("runner.cc")
	add_files("{}")
	    add_rcflags({{"--extern=cheriot", "-L./build/.objs/libcheriot/cheriot/cheriot/release/cheriot/riscv32cheriot-unknown-cheriotrtos/release" }}, {{force = true}})


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
            libcheriot_manifest_path.display(),
            test_rs_path.file_name().expect("Must have a test file!").display(),
        );

        info!(self, "generating xmake config (", out_dir.display(), ")...");
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
