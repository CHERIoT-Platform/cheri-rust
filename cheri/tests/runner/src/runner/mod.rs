mod build;
mod check;
mod cmd;
mod log;

use std::path::{Path, PathBuf};

use colored::Colorize;
use log::*;

/// `cheriot-runner` is a helper to run Rust tests using a simulator (or a real device)
/// targeting CHERIoT's RTOS.
///
/// Notes: requires `xmake`, `nm`, `git`, `rustc` and a CHERIoT-enabled `clang`.
#[derive(Debug, clap::Parser)]
pub struct App {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// The path to the CHERIoT-enabled LLVM sysroot.
    #[clap(long = "sysroot", env = "CHERIOT_SYSROOT_DIR")]
    sysroot_dir: PathBuf,

    // /// The executable that will run the resulting firmware. -- unsupported for now.
    // #[clap(long = "runner", env = "CHERIOT_SIM_PATH")]
    // runner_path: PathBuf,
    /// The `xmake` executable.
    #[clap(long = "xmake", default_value = "xmake")]
    xmake_path: PathBuf,

    /// The `rustc` executable.
    #[clap(long = "rustc", default_value = "rustc")]
    rustc_path: PathBuf,

    /// The `rustc` toolchain name to use.
    #[clap(long = "rustc-toolchain")]
    rustc_toolchain: Option<String>,

    /// The name to pass `rustc` to compile for CHERIoT.
    #[clap(long = "rustc-target", default_value = "riscv32cheriot-unknown-cheriotrtos")]
    rustc_target: String,

    /// The `git` executable.
    #[clap(long = "git", default_value = "git")]
    git_path: PathBuf,

    /// The URL to the cheriot-rtos repository.
    #[clap(
        long = "rtos-repo-url",
        default_value = "https://github.com/CHERIoT-Platform/cheriot-rtos.git"
    )]
    cheriot_rtos_repo_url: String,

    /// The URL to the cheriot-rtos repository.
    #[clap(long = "rtos-repo-branch", default_value = Some("main"))]
    cheriot_rtos_repo_branch: Option<String>,

    /// Paths to Rust files or directories containing Rust files to compile and link together to
    /// create a test firmware.
    #[clap(default_value = ".")]
    tests: Vec<PathBuf>,
}

impl App {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        let tests = self.discover()?;

        if tests.is_empty() {
            warnln!(self, "no tests to run: exiting without doing anything");
            return Ok(());
        }

        self.check_deps()?;

        let out_dir = std::env::temp_dir().join("cheri-runner");

        // Clone the RTOS code.
        let rtos_dir = out_dir.join("rtos");
        if !rtos_dir.exists() {
            self.git(
                &self.cheriot_rtos_repo_url,
                self.cheriot_rtos_repo_branch.as_ref().map(|v| v.as_str()),
                &rtos_dir,
            )?;
        } else {
            if !rtos_dir.is_dir() {
                anyhow::bail!(
                    "rtos clone path {} exists but is not a directory",
                    rtos_dir.display()
                )
            }
            info!(self, "cloning the  `rtos` code...");
            infoln!(self, "already exists".bright_green());
        }

        let build_artefacts_dir = out_dir.join("objs");

        // Unfortunately we need to copy libcheriot and the tests somewhere near where the tests
        // will run, so that xmake won't fail creating temp dirs..
        let libcheriot_dir =
            std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("libcheriot");

        info!(self, "copying libcheriot to ", out_dir.display(), "...");
        let mut cmd = std::process::Command::new("cp");
        cmd.arg("-r");
        cmd.args([&libcheriot_dir, &out_dir]);
        cmd.output()?;
        info!(self, "ok");

        let libcheriot_manifest_path = out_dir.join("libcheriot/Cargo.toml");

        let mut test_dirs = vec![];
        let mut test_names = vec![];

        for test_rs_path in &tests {
            let (test_dir, test_name, test_path) =
                self.generate_test_env(&build_artefacts_dir, &test_rs_path)?;

            // Generate the C++ shim for this test.
            self.generate_runner(&format!("test_{test_name}"), &test_dir)?;
            // Generate the xmake config for this test.
            self.generate_xmake_config(
                &test_path,
                &libcheriot_manifest_path,
                &rtos_dir.join("sdk"),
                &test_dir,
            )?;

            self.xmake_run_config(&test_dir)?;

            self.xmake_build_libcheriot(&test_dir)?;

            // It is useful to see the generated LLVM IR for the test.
            self.generate_llvm_ir(&test_path, &test_dir)?;

            self.xmake_build(&test_dir)?;

            test_dirs.push(test_dir);
            test_names.push(test_name);
        }

        let total = test_dirs.len();
        let mut fails = 0;

        for (i, (path, name)) in test_dirs.iter().zip(test_names).enumerate() {
            let output = self.xmake_run(&path);
            print!("[{}/{total}] test {} ... ", i + 1, name.bright_white());

            if output.is_err() || output.as_ref().is_ok_and(|o| o.status.code() != Some(0)) {
                fails += 1;
                println!("{}", "fail".red())
            } else {
                println!("{}", "ok".green());
                continue;
            }

            if !self.verbosity.is_silent()
                && let Ok(output) = output
            {
                let mut stdout = String::from_utf8(output.stdout)?;

                let stderr = String::from_utf8(output.stderr)?;

                traceln!(self, "--- begin stdout from test runner --");
                traceln!(self, stdout);
                traceln!(self, "--- end stdout from test runner --");
                traceln!(self, "--- begin stderr from test runner --");
                traceln!(self, stderr);
                traceln!(self, "--- end stderr from test runner --");

                let start_needle = "@rust-test-runner-sync-start\n";
                let good_end_needle = "@rust-test-runner-sync-end";
                if let Some(prefix) = stdout.find(start_needle) {
                    let mut stdout = stdout.split_off(prefix + start_needle.len());
                    if let Some(suffix) = stdout.rfind(good_end_needle) {
                        _ = stdout.split_off(suffix);
                    }

                    if stdout.is_empty() {
                        infoln!(self, "");
                    } else {
                        println!("-- begin stdout from test {name}");
                        print!("{stdout}");
                        println!(
                            "\n-- end stdout from test {name} (run with more verbosity to see the entire output)"
                        );
                    }
                }
            }
        }

        println!("ran {} tests, {} failed and {} succeeded", total, fails, total - fails);

        if fails != 0 { anyhow::bail!("Some test failed!") } else { Ok(()) }
    }

    fn xmake_run_config(&self, dir: &Path) -> anyhow::Result<()> {
        let sdk_flag = format!("--sdk={}", self.sysroot_dir.canonicalize().unwrap().display(),);
        let rc_flag = format!("--rc={}", self.rustc_path.canonicalize().unwrap().display());
        self.xmake_strict(dir, &["config", &sdk_flag, &rc_flag]).map(|_| ())
    }

    fn xmake_build(&self, dir: &Path) -> anyhow::Result<()> {
        self.xmake_strict(dir, &["build"]).map(|_| ())
    }

    fn xmake_build_libcheriot(&self, dir: &Path) -> anyhow::Result<()> {
        self.xmake_strict(dir, &["build", "libcheriot"]).map(|_| ())
    }

    fn xmake_run(&self, dir: &Path) -> anyhow::Result<std::process::Output> {
        self.xmake(dir, &["run"])
    }

    fn generate_llvm_ir(&self, test_path: &PathBuf, test_out_dir: &PathBuf) -> anyhow::Result<()> {
        self.rustc(
            test_out_dir,
            &test_path,
            &[
                "--emit=llvm-ir",
                "--extern=cheriot",
                "-L./build/.objs/libcheriot/cheriot/cheriot/release/cheriot/riscv32cheriot-unknown-cheriotrtos/release",
            ],
        )
    }
}
