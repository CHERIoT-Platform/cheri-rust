//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]
#![no_std]

pub struct SelfRef {
    self_ref: *mut SelfRef,
    _pin: core::marker::PhantomPinned,
}

// CHECK-LABEL: @test_self_ref(
// CHECK-NOT: noalias
#[no_mangle]
pub unsafe fn test_self_ref(s: &mut SelfRef) {
    (*s.self_ref).self_ref = core::ptr::null_mut();
}
