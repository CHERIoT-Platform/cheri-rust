#![feature(never_type)]
extern crate alloc;

pub mod types;

pub use options::*;
pub use types::*;

pub fn run_tests(tests: &[&TestDescAndFn]) {
    let total = tests.len();
    let mut ignored = 0;
    let mut pass = 0;

    println!("running {} tests...", total);

    for test in tests {
        let TestDescAndFn { desc, testfn } = make_owned_test(test);

        // FIXME: support should_panic (#144)
        if desc.ignore || !matches!(desc.should_panic, ShouldPanic::No) {
            ignored += 1;
            continue;
        }

        println!("{} ({}:{}) ... ", test.desc.name, test.desc.source_file, test.desc.start_line);

        let Runnable::Test(runnable) = testfn.into_runnable();
        let result = runnable.run();

        if result.is_err() {
            // FIXME: support Result failures (#145)
            println!("{}", result.unwrap_err());
            panic!("A test failed!")
        } else {
            pass += 1;
        }
    }

    if total != ignored + pass {
        panic!("Test count mismatch");
    }

    println!("[OK] total={} ignored={} pass={} ", tests.len(), ignored, pass);
}

fn make_owned_test(test: &&TestDescAndFn) -> TestDescAndFn {
    match test.testfn {
        StaticTestFn(f) => TestDescAndFn { testfn: StaticTestFn(f), desc: test.desc.clone() },
    }
}
