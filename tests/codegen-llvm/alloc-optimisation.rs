//@ compile-flags: -Copt-level=3
//@ ignore-riscv32cheriot-unknown-cheriotrtos FIXME: See CHERIoT-Platform/cheri-rust/issues/64

#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::boxed::Box;

#[no_mangle]
pub fn alloc_test(data: u32) {
    // CHECK-LABEL: @alloc_test
    // CHECK-NEXT: start:
    // CHECK-NEXT: ; call __rustc::__rust_no_alloc_shim_is_unstable_v2
    // CHECK-NEXT: tail call void @_R{{.+}}__rust_no_alloc_shim_is_unstable_v2()
    // CHECK-NEXT: ret void
    let x = Box::new(data);
    drop(x);
}
