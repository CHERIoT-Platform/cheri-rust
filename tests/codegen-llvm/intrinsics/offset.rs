//@ compile-flags: -Copt-level=3 -C no-prepopulate-passes

#![crate_type = "lib"]
#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::offset;

// CHECK-LABEL: @offset_zst
// CHECK-SAME: (ptr[[ADDRSPACE]] noundef %p, [[SIZE:i[0-9]+]] noundef{{( signext)?}} %d)
#[no_mangle]
pub unsafe fn offset_zst(p: *const (), d: usize) -> *const () {
    // CHECK-NOT: getelementptr
    // CHECK: ret ptr[[ADDRSPACE]] %p
    offset(p, d)
}

// CHECK-LABEL: @offset_isize
// CHECK-SAME: (ptr[[ADDRSPACE]] noundef %p, [[SIZE]] noundef{{( signext)?}} %d)
#[no_mangle]
pub unsafe fn offset_isize(p: *const u32, d: isize) -> *const u32 {
    // CHECK: %[[R:.*]] = getelementptr inbounds i32, ptr[[ADDRSPACE]] %p, [[SIZE]] %d
    // CHECK-NEXT: ret ptr[[ADDRSPACE]] %[[R]]
    offset(p, d)
}

// CHECK-LABEL: @offset_usize
// CHECK-SAME: (ptr[[ADDRSPACE]] noundef %p, [[SIZE]] noundef{{( signext)?}} %d)
#[no_mangle]
pub unsafe fn offset_usize(p: *const u64, d: usize) -> *const u64 {
    // CHECK: %[[R:.*]] = getelementptr inbounds{{( nuw)?}} i64, ptr[[ADDRSPACE]] %p, [[SIZE]] %d
    // CHECK-NEXT: ret ptr[[ADDRSPACE]] %[[R]]
    offset(p, d)
}
