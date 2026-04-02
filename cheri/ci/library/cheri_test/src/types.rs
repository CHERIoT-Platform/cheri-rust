//! These definitions are extracted from `library/test`, and are necessary as they
//! are expected by the compiler during #[test] expansion. The only modification
//! is removal of some enum members for tests we do not support.

use alloc::string::String;
use alloc::{fmt, format};
use core::convert::Infallible;

pub use TestFn::*;
pub use TestName::*;

pub mod options {

    /// Whether test is expected to panic or not
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum ShouldPanic {
        No,
        Yes,
        YesWithMessage(&'static str),
    }
}

// The name of a test. By convention this follows the rules for rust
// paths; i.e., it should be a series of identifiers separated by double
// colons. This way if some test runner wants to arrange the tests
// hierarchically it may.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TestName {
    StaticTestName(&'static str),
}

impl TestName {
    pub fn as_slice(&self) -> &str {
        match *self {
            StaticTestName(s) => s,
        }
    }
}
impl fmt::Display for TestName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_slice(), f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestType {
    /// Integration-style tests are expected to be in the `tests` folder of the crate.
    IntegrationTest,
}

pub enum TestFn {
    StaticTestFn(fn() -> Result<(), String>),
}

impl TestFn {
    pub fn into_runnable(self) -> Runnable {
        match self {
            StaticTestFn(f) => Runnable::Test(RunnableTest::Static(f)),
        }
    }
}

impl fmt::Debug for TestFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            StaticTestFn(..) => "StaticTestFn(..)",
        })
    }
}

pub enum Runnable {
    Test(RunnableTest),
}

pub enum RunnableTest {
    Static(fn() -> Result<(), String>),
}

impl RunnableTest {
    pub(crate) fn run(self) -> Result<(), String> {
        match self {
            RunnableTest::Static(f) => __rust_begin_short_backtrace(f),
        }
    }
}

// The definition of a single test. A test runner will run a list of
// these.
#[derive(Clone, Debug)]
pub struct TestDesc {
    pub name: TestName,
    pub ignore: bool,
    pub ignore_message: Option<&'static str>,
    pub source_file: &'static str,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub should_panic: options::ShouldPanic,
    pub compile_fail: bool,
    pub no_run: bool,
    pub test_type: TestType,
}

#[derive(Debug)]
pub struct TestDescAndFn {
    pub desc: TestDesc,
    pub testfn: TestFn,
}

// This specific function name is used, on systems that support it, as a "needle"
// to clean up the backtraces when printing them without RUST_BACKTRACE=full (see
// implementation in library/std/src/sys/backtrace.rs).
//
// We don't support backtraces right now.
#[inline(never)]
fn __rust_begin_short_backtrace<T, F: FnOnce() -> T>(f: F) -> T {
    let result = f();

    // prevent this frame from being tail-call optimised away
    core::hint::black_box(result)
}

#[derive(PartialEq, Eq, Clone)]
pub enum ExitCode {
    Int(u8),
    Msg(String),
}

impl alloc::fmt::Debug for ExitCode {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        let mut d = f.debug_tuple("exit status: ");

        let d = match self {
            ExitCode::Int(i) => d.field(i),
            ExitCode::Msg(msg) => d.field(msg),
        };

        d.finish()
    }
}

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode::Int(0 as _);
    pub const FAILURE: ExitCode = ExitCode::Int(1 as _);
}

pub trait Termination {
    /// Is called to get the representation of the value as status code.
    /// This status code is returned to the operating system.
    fn report(self) -> ExitCode;
}

impl Termination for () {
    #[inline]
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}

impl Termination for ! {
    fn report(self) -> ExitCode {
        self
    }
}

impl Termination for Infallible {
    fn report(self) -> ExitCode {
        match self {}
    }
}

impl Termination for ExitCode {
    #[inline]
    fn report(self) -> ExitCode {
        self
    }
}

impl<T: Termination, E: alloc::fmt::Debug> Termination for Result<T, E> {
    fn report(self) -> ExitCode {
        match self {
            Ok(val) => val.report(),
            Err(err) => ExitCode::Msg(format!("{err:?}")),
        }
    }
}

#[allow(unused)]
/// Invoked when unit tests terminate. Returns `Result::Err` if the test is
/// considered a failure. By default, invokes `report()` and checks for a `0`
/// result.
pub fn assert_test_result<T: Termination>(term: T) -> Result<(), String> {
    match term.report() {
        ExitCode::Int(i) if i == 0 => Ok(()),
        e => Err(format!("error: {e:?}")),
    }
}
