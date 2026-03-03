#![no_std]
#![feature(never_type)]
extern crate alloc;

pub mod global_allocator;
pub mod panic;
pub mod print;

use print::*;

pub mod types;

pub use options::*;
pub use types::*;

unsafe extern "C" {
    fn cheriot_exit(code: u32) -> !;
}

pub fn run_tests(tests: &[&TestDescAndFn]) {
    let mut pass = 0;
    let fail = 0;
    let mut ignored = 0;

    println!("running {} tests...", tests.len());

    for test in tests {
        let TestDescAndFn { desc, testfn } = make_owned_test(test);

        if desc.ignore {
            ignored += 1;
            continue;
        }

        let Runnable::Test(runnable) = testfn.into_runnable();

        let should_panic = !matches!(desc.should_panic, ShouldPanic::No);

        println!("{} ...", test.desc.name.as_slice());

        let result = crate::panic::try_run(|| runnable.run(), should_panic);

        let did_panic = result.is_err();

        if should_panic == did_panic {
            pass += 1;
            println!("{} ... OK", test.desc.name.as_slice());
        } else {
            // fail += 1;
            println!("{} ... FAIL", test.desc.name.as_slice());
            unsafe { cheriot_exit(1) }
        }
    }

    println!("total: {}, pass: {}, fail: {}, ignored: {}", tests.len(), pass, fail, ignored);

    if fail > 0 {
        unsafe { cheriot_exit(1) }
    }
}

fn make_owned_test(test: &&TestDescAndFn) -> TestDescAndFn {
    match test.testfn {
        StaticTestFn(f) => TestDescAndFn { testfn: StaticTestFn(f), desc: test.desc.clone() },
    }
}
