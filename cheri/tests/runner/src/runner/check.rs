//! Functions to discover tests and check the validity of the executables the runner needs.

use std::collections::HashSet;
use std::fs::read_dir;
use std::path::PathBuf;

use colored::Colorize;

use super::App;
use crate::runner::log::*;

impl App {
    pub(crate) fn discover<'a>(&'a self) -> anyhow::Result<Vec<PathBuf>> {
        let mut res = HashSet::new();

        fn is_single_valid_test(p: &PathBuf) -> bool {
            !std::fs::canonicalize(p).is_ok_and(|v| v.starts_with(env!("CARGO_MANIFEST_DIR")))
                && p.is_file()
                && p.extension().is_some_and(|v| v == "rs")
        }

        fn walk_dir(dir: &PathBuf) -> anyhow::Result<HashSet<PathBuf>> {
            let mut res = HashSet::new();
            for item in read_dir(dir)? {
                let item = item?;
                let meta = item.metadata()?;
                let path = item.path();

                if is_single_valid_test(&path) {
                    res.insert(path);
                } else if meta.is_dir() {
                    for item in walk_dir(&path)? {
                        res.insert(item);
                    }
                }
            }
            Ok(res)
        }

        info!(self, "discovering tests to run...");
        for test in &self.tests {
            if !test.exists() {
                info!(self, "failed!".bright_red());
                anyhow::bail!("test '{}' does not exist", test.display())
            }

            if is_single_valid_test(test) {
                res.insert(test.clone());
            } else if test.is_dir() {
                let items = walk_dir(test).map_err(|e| {
                    info!(self, "failed!".bright_red());
                    anyhow::anyhow!("{e}")
                })?;
                for test in items {
                    res.insert(test);
                }
            }
        }

        infoln!(self, "ok".bright_green());
        infoln!(self, "found ", res.len(), " test(s) to run");

        Ok(res.into_iter().collect())
    }

    pub(crate) fn check_deps(&self) -> anyhow::Result<()> {
        // // Check if we have a runner. -- unsupported for now.
        //self.check_executable(&self.runner_path)?;

        // Check if we have `xmake`.
        self.check_executable(&self.xmake_path)?;

        // Check if we have `rustc`.
        self.check_executable(&self.rustc_path)?;

        // Check if `rustc` has the given toolchain.
        if let Some(ref toolchain) = self.rustc_toolchain {
            info!(self, "checking `", self.rustc_path.display(), " +", toolchain, "`...");

            let mut cmd = std::process::Command::new(&self.rustc_path);
            cmd.args([toolchain.as_str(), "-h"]);
            let res = cmd.output();

            if let Err(e) = res {
                infoln!(self, "failed".bright_red());
                anyhow::bail!("failed to check `rustc` toolchain `{toolchain}` ({e})")
            }
            infoln!(self, "ok".bright_green());
        }

        // Check if we have `git`.
        self.check_executable(&self.git_path)?;

        Ok(())
    }

    fn check_executable(&self, maybe_exe: &PathBuf) -> anyhow::Result<()> {
        info!(self, "checking `", maybe_exe.display(), "`...");
        self.can_run(maybe_exe).map_err(|e| {
            infoln!(self, "failed".bright_red());
            anyhow::anyhow!("failed to check `{}` dependency: {e:}", maybe_exe.display())
        })?;
        infoln!(self, "ok".bright_green());
        Ok(())
    }

    fn can_run(&self, maybe_exe: &PathBuf) -> anyhow::Result<()> {
        let mut cmd = std::process::Command::new(&maybe_exe);
        cmd.arg("-h");
        let res = cmd.output();

        if let Err(e) = res {
            anyhow::bail!("failed to run {cmd:?} ({e})")
        }

        Ok(())
    }
}
