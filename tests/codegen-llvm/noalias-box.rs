//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]
#![no_std]
extern crate alloc;
use alloc::boxed::Box;

// CHECK-LABEL: @box_should_have_noalias_by_default(
// CHECK: noalias
#[no_mangle]
pub fn box_should_have_noalias_by_default(_b: Box<u8>) {}
