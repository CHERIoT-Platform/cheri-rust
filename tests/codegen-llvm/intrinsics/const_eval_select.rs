//@ compile-flags: -C no-prepopulate-passes -Copt-level=0

#![crate_type = "lib"]
#![no_std]
#![feature(const_eval_select)]
#![feature(core_intrinsics)]

use core::intrinsics::const_eval_select;

const fn foo(_: i32) -> i32 {
    1
}

#[no_mangle]
pub fn hi(n: i32) -> i32 {
    n
}

#[no_mangle]
pub unsafe fn hey() {
    // CHECK: call[[ADDRSPACE]] i32 @hi(i32
    const_eval_select((42,), foo, hi);
}
