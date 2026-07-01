//! # test-runner
//! This tool is designed to simplify running various `rustc` test suites
//! on our CHERIoT target, as a replacement for bootstrap (`x.py`).
//!
//! This tool currently supports:
//!     * `coretests`
//!
//! We require such a tool for various reasons, including:
//!     * `libtest` expects `std` features we do not or cannot support.
//!     * Suites such as `coretests` are compiled to a single binary which
//!       is too large for our platform.
//!     * Suites such as `compiletest/ui` want to link thousands of individual
//!       executables which takes a very long time on our platform.
//!
//! This tool is designed to be used in CI. If you want to run it locally, be
//! aware of the following:
//!     * You should run this tool from the root directory of the `cheri-rust`
//!       repo. If you are elsewhere, you must provide a `--root-dir` argument.
//!     * You must have built a compiler and "sysroot" for our "facade" target,
//!       e.g. `x build compiler std --target riscv32cheriot-unknown-cheriotrtos.facade`.
//!     * This tool does not detect changes to the "sysroot" crates ("core", "std", etc.).
//!       You need to rebuild the compiler and sysroot with the command above.
//!     * Other changes, except changes to our libtest or the test suite itself, will not
//!       be detected. The "--clean" argument will remove `cargo` and `xmake` build artifacts.

use std::path::PathBuf;

use clap::Parser;

mod cargo;
mod results;
mod runner;

#[derive(Parser)]
struct Args {
    /// E.g. "coretests", "alloctests"
    suite: String,

    /// Specify a single module. If omitted will run all modules. E.g. "iter_num", "fmt"
    module: Option<String>,

    /// Simulator to use for test execution
    #[arg(long, default_value = "cheriot_sim", value_name = "BIN")]
    simulator: PathBuf,

    /// The root of the rustc project. If not set, assumes current working directory
    #[arg(long, default_value = ".", value_name = "DIR")]
    root_dir: PathBuf,

    /// Clean build directories
    #[arg(long)]
    clean: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // a wrapper around cargo with useful commands
    let cargo = cargo::Cargo::new(&args.root_dir);

    if args.clean {
        // runs `cargo clean` on sysroot/alloc and cleans up xmake directories
        cargo.clean()?;
    }

    let modules = match args.module {
        Some(module) => Vec::from([module]),
        // we extract all the `test_` features from the crate metadata
        None => cargo.get_features(&args.suite).unwrap(),
    };

    // for each module, build a test executable and run it through the
    // simulator, then fold the results.
    // TODO: it should be possible to parallelise
    let results = modules
        .iter()
        .map(|module| {
            let executable = cargo.build_test_executable(&args.suite, module)?;
            let runner = runner::Runner::new(&args.simulator, executable);
            let results = runner.run()?;
            Ok(results)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .fold(results::Results::default(), |acc, r| acc + r);

    println!("\n\n{}", results);

    let failures = results.get_failures();

    if !failures.is_empty() {
        println!(
            "\nFailing:\n{}\n",
            failures.iter().map(|test| format!("  {}", test,)).collect::<Vec<_>>().join("\n")
        );
        anyhow::bail!("test suite unsuccessful")
    }

    Ok(())
}
