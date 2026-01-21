//@ aux-build:nounwind.rs
//@ compile-flags: -C no-prepopulate-passes -C panic=abort -C metadata=a
//@ ignore-android

#![crate_type = "lib"]
#![no_std]

extern crate nounwind;

#[no_mangle]
pub fn foo() {
    nounwind::bar();
    // CHECK: @foo() unnamed_addr[[ADDRSPACE]] #0
    // CHECK: @bar() unnamed_addr[[ADDRSPACE]] #0
    // CHECK: attributes #0 = { {{.*}}nounwind{{.*}} }
}
