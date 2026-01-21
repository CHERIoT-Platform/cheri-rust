#![crate_type = "lib"]
#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;
use alloc::boxed::Box;
use core::intrinsics::*;

pub unsafe fn test_volatile_order() {
    let mut a: Box<u8> = Box::new(0);
    // CHECK: load volatile
    let x = volatile_load(&*a);
    // CHECK: load volatile
    let x = volatile_load(&*a);
    // CHECK: store volatile
    volatile_store(&mut *a, 12);
    // CHECK: store volatile
    unaligned_volatile_store(&mut *a, 12);
    // CHECK: llvm.memset.{{p0|p200}}
    volatile_set_memory(&mut *a, 12, 1)
}
