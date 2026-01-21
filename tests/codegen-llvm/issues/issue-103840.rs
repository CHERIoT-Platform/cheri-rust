//@ compile-flags: -Copt-level=3
#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn foo(t: &mut Vec<usize>) {
    // CHECK-NOT: __rust_dealloc
    let mut taken = core::mem::take(t);
    taken.pop();
    *t = taken;
}
