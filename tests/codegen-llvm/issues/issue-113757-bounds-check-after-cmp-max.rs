// in Rust 1.73, -O and opt-level=3 optimizes differently
//@ compile-flags: -C opt-level=3
#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::cmp::max;

// CHECK-LABEL: @foo
// CHECK-NOT: slice_index_fail
// CHECK-NOT: unreachable
#[no_mangle]
pub fn foo(v: &mut Vec<u8>, size: usize) -> Option<&mut [u8]> {
    if v.len() > max(1, size) {
        let start = v.len() - size;
        Some(&mut v[start..])
    } else {
        None
    }
}
