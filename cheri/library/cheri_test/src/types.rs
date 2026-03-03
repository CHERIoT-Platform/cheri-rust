use alloc::borrow::Cow;
use alloc::string::String;
use alloc::{fmt, format};
use core::convert::Infallible;

pub use NamePadding::*;
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

    /// Whether should console output be colored or not
    #[derive(Copy, Clone, Default, Debug)]
    pub enum ColorConfig {
        #[default]
        AutoColor,
        AlwaysColor,
        NeverColor,
    }

    /// Format of the test results output
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    pub enum OutputFormat {
        /// Verbose output
        Pretty,
        /// Quiet output
        #[default]
        Terse,
        /// JSON output
        Json,
        /// JUnit output
        Junit,
    }

    /// Whether ignored test should be run or not
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum RunIgnored {
        Yes,
        No,
        /// Run only ignored tests
        Only,
    }

    #[derive(Clone, Copy)]
    pub enum RunStrategy {
        /// Runs the test in the current process, and sends the result back over the
        /// supplied channel.
        InProcess,

        /// Spawns a subprocess to run the test, and sends the result back over the
        /// supplied channel. Requires `argv[0]` to exist and point to the binary
        /// that's currently running.
        SpawnPrimary,
    }

    /// Options for the test run defined by the caller (instead of CLI arguments).
    /// In case we want to add other options as well, just add them in this struct.
    #[derive(Copy, Clone, Debug)]
    pub struct Options {
        pub display_output: bool,
        pub panic_abort: bool,
    }

    impl Options {
        pub fn new() -> Options {
            Options { display_output: false, panic_abort: false }
        }

        pub fn display_output(mut self, display_output: bool) -> Options {
            self.display_output = display_output;
            self
        }

        pub fn panic_abort(mut self, panic_abort: bool) -> Options {
            self.panic_abort = panic_abort;
            self
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NamePadding {
    PadNone,
    PadOnRight,
}

// The name of a test. By convention this follows the rules for rust
// paths; i.e., it should be a series of identifiers separated by double
// colons. This way if some test runner wants to arrange the tests
// hierarchically it may.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TestName {
    StaticTestName(&'static str),
    // DynTestName(String),
    AlignedTestName(Cow<'static, str>, NamePadding),
}

impl TestName {
    pub fn as_slice(&self) -> &str {
        match *self {
            StaticTestName(s) => s,
            // DynTestName(ref s) => s,
            AlignedTestName(ref s, _) => s,
        }
    }

    pub fn padding(&self) -> NamePadding {
        match self {
            &AlignedTestName(_, p) => p,
            _ => PadNone,
        }
    }

    pub fn with_padding(&self, padding: NamePadding) -> TestName {
        let name = match *self {
            TestName::StaticTestName(name) => Cow::Borrowed(name),
            // TestName::DynTestName(ref name) => Cow::Owned(name.clone()),
            TestName::AlignedTestName(ref name, _) => name.clone(),
        };

        TestName::AlignedTestName(name, padding)
    }
}
impl fmt::Display for TestName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_slice(), f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestType {
    /// Unit-tests are expected to be in the `src` folder of the crate.
    UnitTest,
    /// Integration-style tests are expected to be in the `tests` folder of the crate.
    IntegrationTest,
    /// Doctests are created by the `librustdoc` manually, so it's a different type of test.
    DocTest,
    /// Tests for the sources that don't follow the project layout convention
    /// (e.g. tests in raw `main.rs` compiled by calling `rustc --test` directly).
    Unknown,
}

pub enum TestFn {
    StaticTestFn(fn() -> Result<(), String>),
    // StaticBenchFn(fn(&mut Bencher) -> Result<(), String>),
    // StaticBenchAsTestFn(fn(&mut Bencher) -> Result<(), String>),
    // DynTestFn(Box<dyn FnOnce() -> Result<(), String> + Send>),
    // DynBenchFn(Box<dyn Fn(&mut Bencher) -> Result<(), String> + Send>),
    // DynBenchAsTestFn(Box<dyn Fn(&mut Bencher) -> Result<(), String> + Send>),
}

impl TestFn {
    pub fn padding(&self) -> NamePadding {
        match *self {
            StaticTestFn(..) => PadNone,
            // StaticBenchFn(..) => PadOnRight,
            // StaticBenchAsTestFn(..) => PadNone,
            // DynTestFn(..) => PadNone,
            // DynBenchFn(..) => PadOnRight,
            // DynBenchAsTestFn(..) => PadNone,
        }
    }

    pub fn into_runnable(self) -> Runnable {
        match self {
            StaticTestFn(f) => Runnable::Test(RunnableTest::Static(f)),
            // StaticBenchFn(f) => Runnable::Bench(RunnableBench::Static(f)),
            // StaticBenchAsTestFn(f) => Runnable::Test(RunnableTest::StaticBenchAsTest(f)),
            // DynTestFn(f) => Runnable::Test(RunnableTest::Dynamic(f)),
            // DynBenchFn(f) => Runnable::Bench(RunnableBench::Dynamic(f)),
            // DynBenchAsTestFn(f) => Runnable::Test(RunnableTest::DynamicBenchAsTest(f)),
        }
    }
}

impl fmt::Debug for TestFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            StaticTestFn(..) => "StaticTestFn(..)",
            // StaticBenchFn(..) => "StaticBenchFn(..)",
            // StaticBenchAsTestFn(..) => "StaticBenchAsTestFn(..)",
            // DynTestFn(..) => "DynTestFn(..)",
            // DynBenchFn(..) => "DynBenchFn(..)",
            // DynBenchAsTestFn(..) => "DynBenchAsTestFn(..)",
        })
    }
}

pub enum Runnable {
    Test(RunnableTest),
    // Bench(RunnableBench),
}

pub enum RunnableTest {
    Static(fn() -> Result<(), String>),
    // Dynamic(Box<dyn FnOnce() -> Result<(), String> + Send>),
    // StaticBenchAsTest(fn(&mut Bencher) -> Result<(), String>),
    // DynamicBenchAsTest(Box<dyn Fn(&mut Bencher) -> Result<(), String> + Send>),
}

impl RunnableTest {
    pub(crate) fn run(self) -> Result<(), String> {
        match self {
            RunnableTest::Static(f) => __rust_begin_short_backtrace(f), // __rust_begin_short_backtrace(f),
                                                                        // RunnableTest::Dynamic(f) => f(), //__rust_begin_short_backtrace(f),
                                                                        // RunnableTest::StaticBenchAsTest(f) => {
                                                                        //     crate::bench::run_once(|b| __rust_begin_short_backtrace(|| f(b)))
                                                                        // }
                                                                        // RunnableTest::DynamicBenchAsTest(f) => {
                                                                        //     crate::bench::run_once(|b| __rust_begin_short_backtrace(|| f(b)))
                                                                        // }
        }
    }

    // pub(crate) fn is_dynamic(&self) -> bool {
    //     match self {
    //         RunnableTest::Static(_) => false,
    //         // RunnableTest::StaticBenchAsTest(_) => false,
    //         // RunnableTest::Dynamic(_) => true,
    //         // RunnableTest::DynamicBenchAsTest(_) => true,
    //     }
    // }
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

impl TestDesc {
    pub fn padded_name(&self, column_count: usize, align: NamePadding) -> String {
        let mut name = String::from(self.name.as_slice());
        let fill = column_count.saturating_sub(name.len());
        let pad = " ".repeat(fill);
        match align {
            PadNone => name,
            PadOnRight => {
                name.push_str(&pad);
                name
            }
        }
    }

    /// Returns None for ignored test or tests that are just run, otherwise returns a description of the type of test.
    /// Descriptions include "should panic", "compile fail" and "compile".
    pub fn test_mode(&self) -> Option<&'static str> {
        if self.ignore {
            return None;
        }
        match self.should_panic {
            options::ShouldPanic::Yes | options::ShouldPanic::YesWithMessage(_) => {
                return Some("should panic");
            }
            options::ShouldPanic::No => {}
        }
        if self.compile_fail {
            return Some("compile fail");
        }
        if self.no_run {
            return Some("compile");
        }
        None
    }
}

#[derive(Debug)]
pub struct TestDescAndFn {
    pub desc: TestDesc,
    pub testfn: TestFn,
}

impl TestDescAndFn {
    pub const fn new_doctest(
        test_name: &'static str,
        ignore: bool,
        source_file: &'static str,
        start_line: usize,
        no_run: bool,
        should_panic: bool,
        testfn: TestFn,
    ) -> Self {
        Self {
            desc: TestDesc {
                name: StaticTestName(test_name),
                ignore,
                ignore_message: None,
                source_file,
                start_line,
                start_col: 0,
                end_line: 0,
                end_col: 0,
                compile_fail: false,
                no_run,
                should_panic: if should_panic {
                    options::ShouldPanic::Yes
                } else {
                    options::ShouldPanic::No
                },
                test_type: TestType::DocTest,
            },
            testfn,
        }
    }
}

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
