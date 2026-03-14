//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

// CHECK-LABEL: @should_reserve_once
#[no_mangle]
pub fn should_reserve_once(v: &mut Vec<u8>) {
    // CHECK: tail call[[ADDRSPACE]] void @llvm.assume
    v.try_reserve(3).unwrap();
    // CHECK-NOT: call[[ADDRSPACE]] {{.*}}reserve
    // CHECK-NOT: call[[ADDRSPACE]] {{.*}}do_reserve_and_handle
    // CHECK-NOT: call[[ADDRSPACE]] {{.*}}__rust_alloc(
    v.extend([1, 2, 3]);
}
