//@ compile-flags: -C no-prepopulate-passes -C panic=abort -C force-unwind-tables=n
//@ ignore-windows: unwind tables are required for panics on Windows

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: define{{.*}}void @foo
// CHECK-NOT: attributes #{{.*}} uwtable
#[no_mangle]
fn foo() {
    panic!();
}

// CHECK-NOT: !"uwtable"
