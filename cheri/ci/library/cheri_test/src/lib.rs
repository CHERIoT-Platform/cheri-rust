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

    println!(r#"{{ "type": "suite", "event": "started", "test_count": {} }}"#, total);

    for test in tests {
        let TestDescAndFn { desc, testfn } = make_owned_test(test);

        let name = EscapedString(desc.name.as_slice());

        println!(r#"{{ "type": "test", "event": "started", "name": "{}" }}"#, name);

        if desc.ignore {
            println!(r#"{{ "type": "test", "event": "ignored", "name": "{}" }}"#, name);
            ignored += 1;
            continue;
        }

        let Runnable::Test(runnable) = testfn.into_runnable();
        let result = fold_err(try_run(|| runnable.run()));
        let result = calc_result(&desc, result.err().as_deref());

        match result {
            TestResult::Pass => {
                println!(r#"{{ "type": "test", "event": "ok", "name": "{}" }}"#, name);
                pass += 1;
            }
            TestResult::Fail(msg) => {
                println!(
                    r#"{{ "type": "test", "event": "failed", "name": "{}", "stdout": "{}" }}"#,
                    name,
                    EscapedString(msg)
                );
                fail += 1;
            }
        }
    }

    assert_eq!(total, ignored + pass + fail);

    let result = if fail == 0 { "ok" } else { "failed" };

    println!(r#"{{"type": "suite", "event": "{result}"}}"#);
}

fn calc_result(desc: &TestDesc, err_message: Option<&str>) -> TestResult {
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

/// A formatting utility used to print strings with characters in need of escaping.
/// Base code taken form `libserialize::json::escape_str`
struct EscapedString<S: AsRef<str>>(S);

impl<S: AsRef<str>> std::fmt::Display for EscapedString<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut start = 0;

        for (i, byte) in self.0.as_ref().bytes().enumerate() {
            let escaped = match byte {
                b'"' => "\\\"",
                b'\\' => "\\\\",
                b'\x00' => "\\u0000",
                b'\x01' => "\\u0001",
                b'\x02' => "\\u0002",
                b'\x03' => "\\u0003",
                b'\x04' => "\\u0004",
                b'\x05' => "\\u0005",
                b'\x06' => "\\u0006",
                b'\x07' => "\\u0007",
                b'\x08' => "\\b",
                b'\t' => "\\t",
                b'\n' => "\\n",
                b'\x0b' => "\\u000b",
                b'\x0c' => "\\f",
                b'\r' => "\\r",
                b'\x0e' => "\\u000e",
                b'\x0f' => "\\u000f",
                b'\x10' => "\\u0010",
                b'\x11' => "\\u0011",
                b'\x12' => "\\u0012",
                b'\x13' => "\\u0013",
                b'\x14' => "\\u0014",
                b'\x15' => "\\u0015",
                b'\x16' => "\\u0016",
                b'\x17' => "\\u0017",
                b'\x18' => "\\u0018",
                b'\x19' => "\\u0019",
                b'\x1a' => "\\u001a",
                b'\x1b' => "\\u001b",
                b'\x1c' => "\\u001c",
                b'\x1d' => "\\u001d",
                b'\x1e' => "\\u001e",
                b'\x1f' => "\\u001f",
                b'\x7f' => "\\u007f",
                _ => {
                    continue;
                }
            };

            if start < i {
                f.write_str(&self.0.as_ref()[start..i])?;
            }

            f.write_str(escaped)?;

            start = i + 1;
        }

        if start != self.0.as_ref().len() {
            f.write_str(&self.0.as_ref()[start..])?;
        }

        Ok(())
    }
}
