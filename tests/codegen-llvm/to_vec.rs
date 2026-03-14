//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

// CHECK-LABEL: @copy_to_vec
#[no_mangle]
fn copy_to_vec(s: &[u64]) -> Vec<u64> {
    s.to_vec()
    // CHECK: call[[ADDRSPACE]] void @llvm.memcpy
}
