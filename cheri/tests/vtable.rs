#![no_std]

extern crate alloc;
extern crate cheriot;

use alloc::string::String;

trait Chair {
    fn count_legs(&self) -> u32;
}

struct Monobloc {
    dropme: String,
}

impl Chair for Monobloc {
    fn count_legs(&self) -> u32 {
        4
    }
}

struct CuttyStool {}

impl Chair for CuttyStool {
    fn count_legs(&self) -> u32 {
        3
    }
}

struct RockingChair {}

impl Chair for RockingChair {
    fn count_legs(&self) -> u32 {
        0
    }
}

#[no_mangle]
extern "C" fn test_vtable() -> i32 {
    fn observe_chair(chair: &dyn Chair) -> u32 {
        chair.count_legs()
    }
    core::hint::black_box({
        let chair = Monobloc { dropme: String::new() };
        assert_eq!(observe_chair(&chair), 4);

        let chair = CuttyStool {};
        assert_eq!(observe_chair(&chair), 3);

        let chair = RockingChair {};
        assert_eq!(observe_chair(&chair), 0);
    });

    0
}
