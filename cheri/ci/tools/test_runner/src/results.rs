use std::fmt;
use std::ops::Add;

pub enum FailureMode {
    UnexpectedFail,
    UnexpectedPass,
    UnexpectedIgnore,
}

#[derive(Default)]
pub struct Results {
    total: u32,

    failed: u32,
    ignored: u32,
    passed: u32,

    failed_unexpected: u32,
    ignored_unexpected: u32,
    passed_unexpected: u32,

    failures: Vec<(String, FailureMode)>,
}

impl Results {
    pub fn fail(&mut self) {
        self.total += 1;
        self.failed += 1;
    }

    pub fn ignore(&mut self) {
        self.total += 1;
        self.ignored += 1;
    }

    pub fn pass(&mut self) {
        self.total += 1;
        self.passed += 1;
    }

    pub fn fail_unexpected(&mut self, name: String) {
        self.fail();
        self.failed_unexpected += 1;
        self.failures.push((name, FailureMode::UnexpectedFail))
    }

    pub fn ignore_unexpected(&mut self, name: String) {
        self.ignore();
        self.ignored_unexpected += 1;
        self.failures.push((name, FailureMode::UnexpectedIgnore))
    }

    pub fn pass_unexpected(&mut self, name: String) {
        self.pass();
        self.passed_unexpected += 1;
        self.failures.push((name, FailureMode::UnexpectedPass))
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn get_failures(&self) -> &Vec<(String, FailureMode)> {
        &self.failures
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "total={} passed={} failed={} ignored={}",
            self.total, self.passed, self.failed, self.ignored,
        )
    }
}

impl Add for Results {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut failures = self.failures;
        failures.extend(other.failures);

        Self {
            total: self.total + other.total,

            passed: self.passed + other.passed,
            failed: self.failed + other.failed,
            ignored: self.ignored + other.ignored,

            passed_unexpected: self.passed_unexpected + other.passed_unexpected,
            failed_unexpected: self.failed_unexpected + other.failed_unexpected,
            ignored_unexpected: self.ignored_unexpected + other.ignored_unexpected,

            failures,
        }
    }
}
