//@ compile-flags: -g -C no-prepopulate-passes

#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::string::String;

#[derive(Clone, Copy)]
pub enum EmptyEnum {}

#[no_mangle]
pub fn empty(x: &EmptyEnum) -> EmptyEnum {
    // CHECK: @empty({{.*}}) unnamed_addr[[ADDRSPACE]] #0
    // CHECK-NOT: ret void
    // CHECK: call void @llvm.trap()
    // CHECK: unreachable
    *x
}

pub struct Foo(String, EmptyEnum);

#[no_mangle]
pub fn foo(x: String, y: &EmptyEnum) -> Foo {
    // CHECK: @foo({{.*}}) unnamed_addr[[ADDRSPACE]] #0
    // CHECK-NOT: ret %Foo
    // CHECK: call void @llvm.trap()
    // CHECK: unreachable
    Foo(x, *y)
}

// CHECK: attributes #0 = {{{.*}} noreturn {{.*}}}

// CHECK: DISubprogram(name: "empty", {{.*}} DIFlagNoReturn
// CHECK: DISubprogram(name: "foo", {{.*}} DIFlagNoReturn
