//@ compile-flags: -Copt-level=0
#![crate_type = "lib"]
#![no_std]
#![feature(core_intrinsics)]

// CHECK-LABEL: @mask_ptr
// CHECK-SAME: [[WORD:i[0-9]+]]{{( signext)?}} %mask
#[no_mangle]
pub fn mask_ptr(ptr: *const u16, mask: usize) -> *const u16 {
    // CHECK: call
    // CHECK-SAME: @llvm.ptrmask.{{p0|p0i8|p200}}.[[WORD]](ptr[[ADDRSPACE]] {{%ptr|%1}}, [[WORD]] %mask)
    core::intrinsics::ptr_mask(ptr, mask)
}
