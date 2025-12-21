//@ compile-flags: -Copt-level=3
//@ ignore-riscv32cheriot-unknown-cheriotrtos FIXME: CHERIoT-Platform/cheri-rust/issues/71

#![crate_type = "lib"]
#![feature(core_intrinsics)]
#![no_std]

extern crate alloc;
use alloc::boxed::Box;

// Tests that codegen works properly when enums like `Result<usize, Box<()>>`
// are represented as `{ u64, ptr }`, i.e., for `Ok(123)`, `123` is stored
// as a pointer.

// CHECK-LABEL: @insert_int
#[no_mangle]
pub fn insert_int(x: usize) -> Result<usize, Box<()>> {
    // CHECK: start:
    // CHECK-NEXT: %[[WO_PROV:.+]] = inttoptr [[USIZE:i[0-9]+]] %x to ptr[[ADDRSPACE]]
    // CHECK-NEXT: %[[R:.+]] = insertvalue { [[USIZE]], ptr[[ADDRSPACE]] } { [[USIZE]] 0, ptr[[ADDRSPACE]] poison }, ptr %[[WO_PROV]], 1
    // CHECK-NEXT: ret { [[USIZE]], ptr[[ADDRSPACE]] } %[[R]]
    Ok(x)
}

// CHECK-LABEL: @insert_box
#[no_mangle]
pub fn insert_box(x: Box<()>) -> Result<usize, Box<()>> {
    // CHECK: start:
    // CHECK-NEXT: insertvalue { i{{[0-9]+}}, ptr }
    // CHECK-NEXT: ret
    Err(x)
}

// CHECK-LABEL: @extract_int
// CHECK-NOT: nonnull
// CHECK-SAME: (i{{[0-9]+}} {{[^%]+}} [[DISCRIMINANT:%[0-9]+]], ptr {{[^,]+}} [[PAYLOAD:%[0-9]+]])
#[no_mangle]
pub unsafe fn extract_int(x: Result<usize, Box<()>>) -> usize {
    // CHECK: [[TEMP:%.+]] = ptrtoint ptr[[ADDRSPACE]] [[PAYLOAD]] to [[USIZE:i[0-9]+]]
    // CHECK: ret [[USIZE]] [[TEMP]]
    match x {
        Ok(v) => v,
        Err(_) => core::intrinsics::unreachable(),
    }
}

// CHECK-LABEL: @extract_box
// CHECK-SAME: (i{{[0-9]+}} {{[^%]+}} [[DISCRIMINANT:%[0-9]+]], ptr {{[^%]+}} [[PAYLOAD:%[0-9]+]])
#[no_mangle]
pub unsafe fn extract_box(x: Result<usize, Box<i32>>) -> Box<i32> {
    // CHECK: ret ptr[[ADDRSPACE]] [[PAYLOAD]]
    match x {
        Ok(_) => core::intrinsics::unreachable(),
        Err(e) => e,
    }
}
