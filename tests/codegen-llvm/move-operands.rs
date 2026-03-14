// Verify that optimized MIR only copies `a` once.
//@ compile-flags: -Copt-level=3 -C no-prepopulate-passes

#![crate_type = "lib"]
#![no_std]

type T = [u8; 256];

#[no_mangle]
pub fn f(a: T, b: fn(_: T, _: T)) {
    // CHECK: call[[ADDRSPACE]] void @llvm.memcpy.{{.*}}(ptr[[ADDRSPACE]] align 1 %{{.*}}, ptr[[ADDRSPACE]] align 1 %{{.*}}, {{.*}} 256, i1 false)
    // CHECK-NOT: call[[ADDRSPACE]] void @llvm.memcpy.{{.*}}(ptr[[ADDRSPACE]] align 1 %{{.*}}, ptr[[ADDRSPACE]] align 1 %{{.*}}, {{.*}} 256, i1 false)
    b(a, a)
}
