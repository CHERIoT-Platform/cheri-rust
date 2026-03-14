//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: @loop_skips_vtable_load
#[no_mangle]
pub fn loop_skips_vtable_load(x: &dyn Fn()) {
    // CHECK: load ptr[[ADDRSPACE]], ptr[[ADDRSPACE]] %0{{.*}}, !invariant.load
    // CHECK-NEXT: tail call[[ADDRSPACE]] void %1
    // CHECK-NOT: load ptr[[ADDRSPACE]]
    x();
    for _ in 0..100 {
        // CHECK: tail call[[ADDRSPACE]] void %1
        x();
    }
}
