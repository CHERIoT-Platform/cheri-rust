#![no_std]

extern crate cheriot;

#[no_mangle]
extern "C" fn test_ptrtoint() -> i32 {
    do_it() as i32
}

#[inline(never)]
fn do_it() -> i64 {
    let x = 0;
    core::ptr::addr_of!(x) as i64
}
