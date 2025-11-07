#![no_std]

extern crate cheriot;

#[no_mangle]
extern "C" fn test_black_box() -> i32 {
    core::hint::black_box(0)
}
