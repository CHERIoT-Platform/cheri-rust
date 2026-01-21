//@ compile-flags: -C panic=unwind -C no-prepopulate-passes -Copt-level=0

#![crate_type = "lib"]
#![no_std]

// CHECK: attributes #{{.*}} uwtable
pub fn foo() {}
