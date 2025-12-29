//! Utility functions to run commands.

use std::path::Path;

use colored::Colorize;

use super::App;
use super::log::*;

impl App {
    pub(crate) fn rustc(
        &self,
        cwd: &Path,
        path: &Path,
        extra_flags: &[&str],
    ) -> anyhow::Result<()> {
        let rustc_path = if let Ok(canon) = self.rustc_path.canonicalize() {
            canon
        } else {
            self.rustc_path.clone()
        };
        let mut cmd = std::process::Command::new(rustc_path);
        let toolchain = self.rustc_toolchain.as_ref().map(|v| format!("+{v}"));
        let target = format!("--target={}", self.rustc_target);
        let mut args = if let Some(ref toolchain) = toolchain {
            let mut args = vec![toolchain.as_str()];
            args.append(&mut extra_flags.to_vec());
            args
        } else {
            extra_flags.to_vec()
        };
        args.append(&mut vec!["--edition=2021", path.to_str().unwrap(), "-Copt-level=3", &target]);
        cmd.current_dir(cwd);
        cmd.args(args);

        traceln!(self, "running ", format!("{cmd:?}"));
        info!(self, "building `", path.display(), "`...");
        match cmd.output() {
            Ok(s) => {
                if !s.status.success() {
                    infoln!(self, "failed".bright_red());
                    let out = String::from_utf8_lossy(&s.stderr);
                    traceln!(self, "-- begin `rustc` output --");
                    trace!(self, out);
                    traceln!(self, "-- end `rustc` output -- ");
                    anyhow::bail!(
                        "failed to compile `{}` (run with trace logging level to see output)",
                        path.display()
                    )
                }
            }

            Err(e) => {
                infoln!(self, "failed".bright_red());
                anyhow::bail!(
                    "failed to compile `{}`: {e} (run with trace logging level to see output)",
                    path.display()
                )
            }
        };

        infoln!(self, "ok".bright_green());
        Ok(())
    }

    pub(crate) fn git(
        &self,
        url: &str,
        branch: Option<&str>,
        out_dir: &Path,
    ) -> anyhow::Result<()> {
        let mut cmd = std::process::Command::new(&self.git_path);
        let branch = branch.map(|v| format!("--branch={v}"));
        let out = format!("{}", out_dir.display());
        let mut args = vec!["clone", url, &out, "--recursive", "--depth=2"];
        if let Some(ref branch) = branch {
            args.push(branch);
        }
        cmd.args(&args);

        traceln!(self, "running ", format!("{cmd:?}"));
        info!(self, "cloning into `", url, "`...");

        match cmd.output() {
            Ok(s) => {
                if !s.status.success() {
                    infoln!(self, "failed".bright_red());
                    let out = String::from_utf8_lossy(&s.stderr);
                    traceln!(self, "-- begin `git` output --");
                    trace!(self, out);
                    traceln!(self, "-- end `git` output -- ");
                    anyhow::bail!(
                        "failed to clone `{url}` (run with trace logging level to see output)",
                    )
                }
            }

            Err(e) => {
                infoln!(self, "failed".bright_red());
                anyhow::bail!(
                    "failed to clone `{url}`: {e} (run with trace logging level to see output)",
                )
            }
        };

        infoln!(self, "ok".bright_green());
        Ok(())
    }

    pub(crate) fn xmake(&self, cwd: &Path, args: &[&str]) -> anyhow::Result<std::process::Output> {
        let mut cmd = std::process::Command::new(&self.xmake_path);
        cmd.args(args);
        cmd.current_dir(cwd);

        traceln!(self, "running ", format!("{cmd:?}"));
        let args_str = args.join(" ");
        let xmake_path = self.xmake_path.display();
        info!(self, "running `", xmake_path, " ", args_str, "`...");

        match cmd.output() {
            Ok(s) => {
                infoln!(self, "ok".bright_green());
                return Ok(s);
            }

            Err(e) => {
                infoln!(self, "failed".bright_red());
                anyhow::bail!(
                    "command `{xmake_path} {args_str}` failed: {e} (run with trace logging level to see output)",
                )
            }
        };
    }
    pub(crate) fn xmake_strict(
        &self,
        cwd: &Path,
        args: &[&str],
    ) -> anyhow::Result<std::process::Output> {
        let res = self.xmake(cwd, args)?;

        let args_str = args.join(" ");
        let xmake_path = self.xmake_path.display();
        if !res.status.success() {
            infoln!(self, "failed".bright_red());
            let stdout = String::from_utf8_lossy(&res.stdout);
            traceln!(self, "-- begin `", xmake_path, "` stdout --");
            trace!(self, stdout);
            traceln!(self, "-- end `", xmake_path, "` stdout -- ");

            let stderr = String::from_utf8_lossy(&res.stderr);
            traceln!(self, "-- begin `", xmake_path, "` stderr --");
            trace!(self, stderr);
            traceln!(self, "-- end `", xmake_path, "` stderr -- ");

            anyhow::bail!(
                "command `{xmake_path} {args_str}` failed (run with trace logging level to see output)",
            )
        }
        Ok(res)
    }
}
