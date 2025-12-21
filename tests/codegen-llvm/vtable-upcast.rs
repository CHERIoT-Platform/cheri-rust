//! This file tests that we correctly generate GEP instructions for vtable upcasting.
//@ compile-flags: -C no-prepopulate-passes -Copt-level=0

#![crate_type = "lib"]
#![no_std]

pub trait Base {
    fn base(&self);
}

pub trait A: Base {
    fn a(&self);
}

pub trait B: Base {
    fn b(&self);
}

pub trait Diamond: A + B {
    fn diamond(&self);
}

// CHECK-LABEL: upcast_a_to_base
#[no_mangle]
pub fn upcast_a_to_base(x: &dyn A) -> &dyn Base {
    // Requires no adjustment, since its vtable is extended from `Base`.

    // CHECK: start:
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: ret
    x as &dyn Base
}

// CHECK-LABEL: upcast_b_to_base
#[no_mangle]
pub fn upcast_b_to_base(x: &dyn B) -> &dyn Base {
    // Requires no adjustment, since its vtable is extended from `Base`.

    // CHECK: start:
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: ret
    x as &dyn Base
}

// CHECK-LABEL: upcast_diamond_to_a
#[no_mangle]
pub fn upcast_diamond_to_a(x: &dyn Diamond) -> &dyn A {
    // Requires no adjustment, since its vtable is extended from `A` (as the first supertrait).

    // CHECK: start:
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: ret
    x as &dyn A
}

// CHECK-LABEL: upcast_diamond_to_b
// CHECK-SAME: (ptr[[ADDRSPACE]] [[DATA_PTR:%.+]], ptr[[ADDRSPACE]] align {{[0-9]+}} [[VTABLE_PTR:%[a-zA-Z0-9\.]+]])
#[no_mangle]
pub fn upcast_diamond_to_b(x: &dyn Diamond) -> &dyn B {
    // Requires adjustment, since it's a non-first supertrait.

    // CHECK: start:
    // CHECK-NEXT: [[UPCAST_SLOT_PTR:%.+]] = getelementptr inbounds i8, ptr[[ADDRSPACE]] [[VTABLE_PTR]]
    // CHECK-NEXT: [[UPCAST_VTABLE_PTR:%.+]] = load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] [[UPCAST_SLOT_PTR]]
    // CHECK-NEXT: [[FAT_PTR_1:%.+]] = insertvalue { ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] } poison, ptr[[ADDRSPACE]] [[DATA_PTR]], 0
    // CHECK-NEXT: [[FAT_PTR_2:%.+]] = insertvalue { ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] } [[FAT_PTR_1]], ptr[[ADDRSPACE]] [[UPCAST_VTABLE_PTR]], 1
    // CHECK-NEXT: ret { ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] } [[FAT_PTR_2]]
    x as &dyn B
}

// CHECK-LABEL: upcast_diamond_to_b
#[no_mangle]
pub fn upcast_diamond_to_base(x: &dyn Diamond) -> &dyn Base {
    // Requires no adjustment, since `Base` is the first supertrait of `A`,
    // which is the first supertrait of `Diamond`.

    // CHECK: start:
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: insertvalue
    // CHECK-NEXT: ret
    x as &dyn Base
}
