use std::fmt;
use std::ops::Add;

#[derive(Default)]
pub struct Results {
    total: u32,

    failed: u32,
    ignored: u32,
    passed: u32,

    failures: Vec<String>,
}

impl Results {
    pub fn fail(&mut self, name: String) {
        self.total += 1;
        self.failed += 1;
        self.failures.push(name);
    }

    pub fn ignore(&mut self) {
        self.total += 1;
        self.ignored += 1;
    }

    pub fn pass(&mut self) {
        self.total += 1;
        self.passed += 1;
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn get_failures(&self) -> &Vec<String> {
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

            failures,
        }
    }
}
