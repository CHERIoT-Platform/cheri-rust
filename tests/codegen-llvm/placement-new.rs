//@ compile-flags: -Copt-level=3
//@ compile-flags: -Zmerge-functions=disabled
// ignore-tidy-linelength
//@ ignore-riscv32cheriot-unknown-cheriotrtos See https://github.com/CHERIoT-Platform/cheri-rust/issues/74
#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::boxed::Box;
// Test to check that types with "complex" destructors, but trivial `Default` impls
// are constructed directly into the allocation in `Box::default` and `Arc::default`.
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;

// CHECK-LABEL: @box_default_inplace
#[no_mangle]
pub fn box_default_inplace() -> Box<(String, String)> {
    // CHECK-NOT: alloca
    // CHECK: [[BOX:%.*]] = {{.*}}call {{.*}}__rust_alloc(
    // CHECK-NOT: call void @llvm.memcpy
    // CHECK: ret ptr[[ADDRSPACE]] [[BOX]]
    Box::default()
}

// CHECK-LABEL: @rc_default_inplace
#[no_mangle]
pub fn rc_default_inplace() -> Rc<(String, String)> {
    // CHECK-NOT: alloca
    // CHECK: [[RC:%.*]] = {{.*}}call {{.*}}__rust_alloc(
    // CHECK-NOT: call void @llvm.memcpy
    // CHECK: ret ptr[[ADDRSPACE]] [[RC]]
    Rc::default()
}

// CHECK-LABEL: @arc_default_inplace
#[no_mangle]
pub fn arc_default_inplace() -> Arc<(String, String)> {
    // CHECK-NOT: alloca
    // CHECK: [[ARC:%.*]] = {{.*}}call {{.*}}__rust_alloc(
    // CHECK-NOT: call void @llvm.memcpy
    // CHECK: ret ptr[[ADDRSPACE]] [[ARC]]
    Arc::default()
}
