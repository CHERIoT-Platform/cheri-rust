//@ compile-flags: -C no-prepopulate-passes -C force-unwind-tables=y -Copt-level=0

#![crate_type = "lib"]
#![no_std]

// CHECK: attributes #{{.*}} uwtable
pub fn foo() {}
