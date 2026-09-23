use regex::RegexSet;
use std::fmt;
use std::ops::Add;
use std::sync::LazyLock;

pub enum FailureMode {
    UnexpectedFail(Failure, String),
    UnexpectedPass,
    UnexpectedIgnore,
}

#[derive(Debug, Copy, Clone)]
pub enum Failure {
    MissingLibcall,
    TagViolation,
    BoundsViolation,
    PermitExecuteViolation,
    InstructionSelection,
    Transmute,
    RelocationRange,
    NoThreads,
    FailedAssertion,
    ExplicitPanic,
    OutOfMemory,
    Misalignment,
    Unhandled,
    Unknown,
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const FAILURE_PATTERNS: &[(Failure, &str)] = &[
    (Failure::MissingLibcall, r"undefined symbol: __library_export_libcalls_"),
    (Failure::TagViolation, r"TagViolation"),
    (Failure::BoundsViolation, r"BoundsViolation"),
    (Failure::PermitExecuteViolation, r"PermitExecuteViolation"),
    (Failure::InstructionSelection, r"Cannot select"),
    (Failure::Transmute, r"cannot transmute"),
    (Failure::RelocationRange, r"relocation R_RISCV_CHERIOT\d?_COMPARTMENT_SIZE out of range"),
    (Failure::NoThreads, r"failed to spawn thread"),
    (Failure::FailedAssertion, r"(?m)^assertion.*failed$|^assertion failed:.*$"),
    (Failure::ExplicitPanic, r"explicit panic"),
    (Failure::OutOfMemory, r"Allocator error on alloc: -13"),
    (Failure::Misalignment, r"misaligned pointer dereference"),
    (Failure::Unhandled, r"Unhandled error 0x2"),
];

static PATTERNS: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(FAILURE_PATTERNS.iter().map(|(_, pattern)| pattern)).unwrap());

pub fn categorise(stdout: &str) -> Failure {
    PATTERNS
        .matches(stdout)
        .into_iter()
        .next()
        .map(|i| FAILURE_PATTERNS[i].0)
        .unwrap_or(Failure::Unknown)
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

    pub fn fail_unexpected(&mut self, name: String, stdout: String) {
        self.fail();
        self.failed_unexpected += 1;
        let failure = categorise(&stdout);
        self.failures.push((name, FailureMode::UnexpectedFail(failure, stdout)))
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
