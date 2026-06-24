use std::fs::remove_dir_all;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::Context;
use serde::Deserialize;
use serde_json::{Map, Value};

pub const TARGET_FACADE: &str = "riscv32cheriot-unknown-cheriotrtos.facade";

const TARGET_ABI_CFG: &str = "target.'cfg(target_abi = \"cheriot\")'";

fn linker_config(linker: &str) -> String {
    format!("{TARGET_ABI_CFG}.linker = '{}'", linker)
}

const RUSTFLAGS: &str = concat!(
    "-C embed-bitcode=yes ",
    "-C panic=abort ",
    "-C lto ",
    "-C codegen-units=1 ",
    "-C opt-level=3 ",
    "-A warnings",
);

pub struct Cargo {
    cargo_bin: PathBuf,
    rustc_bin: PathBuf,
    rustc_sysroot: PathBuf,
    alloc_manifest: PathBuf,

    linker: PathBuf,
    linker_cache_dir: PathBuf,
    linker_build_dir: PathBuf,
}

impl Cargo {
    pub(crate) fn new(root_dir: &PathBuf) -> Self {
        Self {
            cargo_bin: root_dir.join("build/host/stage0/bin/cargo"),
            rustc_bin: root_dir.join("build/host/stage1/bin/rustc"),
            rustc_sysroot: root_dir.join("build/host/stage1"),
            alloc_manifest: root_dir.join("library/alloc/Cargo.toml"),

            linker: root_dir.join("cheri/ci/script/linker.sh"),
            linker_cache_dir: root_dir.join("cheri/ci/build/.xmake"),
            linker_build_dir: root_dir.join("cheri/ci/build/build"),
        }
    }

    fn cargo_cmd(&self, subcmd: &str) -> Command {
        let mut cmd = Command::new(&self.cargo_bin);
        cmd.envs([
            ("RUSTC_BOOTSTRAP", "1"),
            ("RUSTC", self.rustc_bin.to_str().unwrap()),
            ("RUSTC_SYSROOT", self.rustc_sysroot.to_str().unwrap()),
            ("RUSTFLAGS", RUSTFLAGS),
        ]);
        #[rustfmt::skip]
        cmd.args([
            subcmd,
            "--manifest-path", self.alloc_manifest.to_str().unwrap(),
            "--frozen"
        ]);

        cmd
    }

    pub(crate) fn clean(&self) -> anyhow::Result<()> {
        let cmd = self.cargo_cmd("clean").output()?;
        if !cmd.status.success() {
            let stderr = String::from_utf8(cmd.stderr)?;
            eprintln!("{}", stderr);
            anyhow::bail!("");
        }
        let _ = remove_dir_all(&self.linker_cache_dir);
        let _ = remove_dir_all(&self.linker_build_dir);
        Ok(())
    }

    pub(crate) fn get_features(&self, package: &String) -> anyhow::Result<Vec<String>> {
        let cmd = self.cargo_cmd("metadata").arg("--no-deps").output()?;

        if !cmd.status.success() {
            let stderr = String::from_utf8(cmd.stderr)?;
            eprintln!("{}", stderr);
            anyhow::bail!("");
        }

        let data = serde_json::from_slice::<Metadata>(cmd.stdout.as_slice())?;

        let package = data.packages.iter().find(|p| p.name == *package).context("uh oh")?;

        let features = package
            .features
            .keys()
            .filter_map(|f| f.strip_prefix("test_").map(|f| f.to_owned()))
            .collect();

        Ok(features)
    }

    pub(crate) fn build_test_executable(
        &self,
        package: &String,
        module: &String,
    ) -> anyhow::Result<PathBuf> {
        let features = format!("compiler-builtins-mem {package}/test_{module}");

        #[rustfmt::skip]
        let mut cmd = self
            .cargo_cmd("build")
            .args([
                "--tests",
                "--target",           TARGET_FACADE,
                "--package",          package,
                "--profile",          "release",
                "--features",         &features,
                "--config",           &linker_config(&self.linker.to_str().unwrap()),
                "--message-format",   "json",
                "--quiet",
                "-Zno-embed-metadata"
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("fail");

        let stdout = cmd.stdout.take().expect("failed to take");
        let stderr = cmd.stderr.take().expect("failed to take");

        // we want to keep this to display in case of a failure, but we don't
        // want to block our stdout reader (and vice versa), so read stderr
        // on a separate thread
        let stderr_thread = std::thread::spawn(move || {
            let mut str = String::new();
            BufReader::new(stderr).read_to_string(&mut str).ok();
            str
        });

        let stdout_reader = BufReader::new(stdout);

        for line in stdout_reader.lines() {
            let line = line?;
            let log = match serde_json::from_str::<Build>(&line) {
                Ok(log) => log,
                Err(_) => {
                    eprintln!("Unhandled: {}", line);
                    continue;
                }
            };

            match log {
                // error messages and such
                Build::Message { message } => {
                    eprintln!("{}", message.rendered);
                    continue;
                }
                // the thing we want
                Build::Artifact { executable: Some(executable) } => {
                    cmd.kill()?;
                    stderr_thread.join().ok();
                    return Ok(PathBuf::from(executable));
                }
                // non-interesting
                Build::Artifact { executable: None } | Build::Finished => {
                    continue;
                }
            }
        }

        cmd.wait()?;
        let stderr = stderr_thread.join().unwrap();
        eprintln!("{stderr}");
        anyhow::bail!("Building test executable failed");
    }
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<MetadataPackage>,
}

#[derive(Deserialize)]
struct MetadataPackage {
    name: String,
    features: Map<String, Value>,
}

#[derive(Deserialize)]
#[serde(tag = "reason")]
enum Build {
    #[serde(rename = "compiler-artifact")]
    Artifact { executable: Option<String> },
    #[serde(rename = "compiler-message")]
    Message { message: BuildMessage },
    #[serde(rename = "build-finished")]
    Finished,
}

#[derive(Deserialize)]
struct BuildMessage {
    rendered: String,
}
