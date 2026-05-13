#![feature(never_type)]

extern crate alloc;

mod panic;
pub mod types;

pub use options::*;
use panic::{set_panic_hook, try_run};
pub use types::*;

#[derive(PartialEq)]
enum TestResult {
    Pass,
    Fail(String),
}

pub fn run_tests(tests: &[&TestDescAndFn]) {
    set_panic_hook();

    let total = tests.len();
    let mut ignored = 0;
    let mut pass = 0;
    let mut fail = 0;

    println!("running {} tests...", total);

    for test in tests {
        let TestDescAndFn { desc, testfn } = make_owned_test(test);

        if desc.ignore {
            ignored += 1;
            continue;
        }

        print!("{} ({}:{}) ... ", desc.name, desc.source_file, desc.start_line);

        let Runnable::Test(runnable) = testfn.into_runnable();
        let result = fold_err(try_run(|| runnable.run()));
        let result = calc_result(desc, result.err().as_deref());

        match result {
            TestResult::Pass => {
                print!("OK\n");
                pass += 1;
            }
            TestResult::Fail(msg) => {
                print!("FAIL\n");
                println!("\n{}\n", msg);
                fail += 1;
            }
        }
    }

    assert_eq!(total, ignored + pass + fail);

    let result = if fail == 0 { "OK" } else { "FAIL" };

    println!("[{}] total={} ignored={} pass={} fail={}", result, total, ignored, pass, fail);

    if fail != 0 {
        std::process::exit(1);
    }
}

fn calc_result(desc: TestDesc, err_message: Option<&str>) -> TestResult {
    match (desc.should_panic, err_message) {
        (ShouldPanic::No, None) | (ShouldPanic::Yes, Some(_)) => TestResult::Pass,
        // e.g. #[should_panic = "expected string"]
        (ShouldPanic::YesWithMessage(msg), Some(err_str)) => {
            if err_str.contains(msg) {
                TestResult::Pass
            } else {
                TestResult::Fail(err_str.to_string())
            }
        }
        (ShouldPanic::No, Some(err_str)) => TestResult::Fail(err_str.to_string()),
        (ShouldPanic::Yes, None) | (ShouldPanic::YesWithMessage(_), None) => {
            TestResult::Fail(format!("Expected test to panic"))
        }
    }
}

fn fold_err<T>(result: Result<Result<T, String>, String>) -> Result<T, String> {
    match result {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e),
    }
}

fn make_owned_test(test: &&TestDescAndFn) -> TestDescAndFn {
    match test.testfn {
        StaticTestFn(f) => TestDescAndFn { testfn: StaticTestFn(f), desc: test.desc.clone() },
    }
}
