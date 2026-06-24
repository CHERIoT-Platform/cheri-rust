use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use anyhow::anyhow;
use serde::Deserialize;

use crate::results::Results;

pub struct Runner<'a> {
    simulator: &'a PathBuf,
    executable: PathBuf,
    expected: Option<u32>,
    finished: Option<u32>,
    results: Results,
}

impl<'a> Runner<'a> {
    pub fn new(simulator: &'a PathBuf, executable: PathBuf) -> Self {
        Self { simulator, executable, expected: None, finished: None, results: Results::default() }
    }

    pub fn run(mut self) -> anyhow::Result<Results> {
        let mut child = self.spawn()?;
        self.handle_output(&mut child)?;
        child.wait()?;

        let expected = self.expected.unwrap();
        let finished = self.finished.unwrap();

        // sanity check
        if finished != expected {
            return Err(anyhow!("Test count mismatch: {finished} finished, {expected} expected",));
        }

        Ok(self.results)
    }

    fn spawn(&self) -> anyhow::Result<Child> {
        Command::new(self.simulator)
            .arg(&self.executable)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("failed to spawn simulator: {e}"))
    }

    fn handle_output(&mut self, child: &mut Child) -> anyhow::Result<()> {
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("no stdout"))?;
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            let log = match serde_json::from_str::<Log>(&line) {
                Ok(log) => log,
                Err(_) => {
                    if line.starts_with("Running file ")
                        || line.starts_with("ELF Entry @ 0x")
                        || line.starts_with("SUCCESS")
                        || line.starts_with("FAILED")
                    {
                        continue;
                    }
                    // maybe we don't want to see this either? it is stdout from tests
                    // which we currently do not capture. for ui tests we will need
                    // to capture this.
                    eprintln!("{line}");
                    continue;
                }
            };
            self.handle_log(log);
        }
        Ok(())
    }

    fn handle_log(&mut self, log: Log) -> () {
        match log {
            Log::Suite { suite } => match suite {
                SuiteEvent::Started { test_count } => {
                    self.expected = Some(test_count);
                }
                SuiteEvent::Ok | SuiteEvent::Failed => {
                    self.finished = Some(self.results.get_total());
                }
            },
            Log::Test { test } => match test.event {
                TestEvent::Started => {
                    // we may want to keep track of which tests are in-flight,
                    // or measure their execution time, handle timeouts, etc.
                    println!("{} ... started", test.name);
                }
                TestEvent::Failed { stdout } => {
                    println!("{} ... FAILED", test.name);
                    println!("{stdout}\n");
                    self.results.fail(test.name);
                }
                TestEvent::Ignored => {
                    println!("{} ... ignored", test.name);
                    self.results.ignore();
                }
                TestEvent::Ok => {
                    println!("{} ... ok", test.name);
                    self.results.pass();
                }
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum SuiteEvent {
    Started { test_count: u32 },
    Ok,
    Failed,
}

#[derive(Deserialize)]
pub struct Test {
    pub name: String,
    #[serde(flatten)]
    pub event: TestEvent,
}

#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum TestEvent {
    Started,
    Ok,
    Ignored,
    Failed { stdout: String },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Log {
    Suite {
        #[serde(flatten)]
        suite: SuiteEvent,
    },
    Test {
        #[serde(flatten)]
        test: Test,
    },
}
