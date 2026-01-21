//@ compile-flags: -C no-prepopulate-passes -Zmir-opt-level=0 -Copt-level=0
//@ ignore-riscv32cheriot-unknown-cheriotrtos See CHERIoT-Platform/cheri-rust/issues/75
#![crate_type = "lib"]
#![no_std]

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i[0-9]+]]
#[no_mangle]
pub fn helper(_: usize) {}

// CHECK-LABEL: @ref_dst
#[no_mangle]
pub fn ref_dst(s: &[u8]) {
    // We used to generate an extra alloca and memcpy to ref the dst, so check that we copy
    // directly to the alloca for "x"
    // CHECK: store ptr[[ADDRSPACE]] %s.0, {{.*}} %x
    // CHECK: [[X1:%[0-9]+]] = getelementptr inbounds i8, {{.*}} %x, {{i32 4|i64 8}}
    // CHECK: store [[USIZE]] %s.1, {{.*}} [[X1]]

    let x = &*s;
    &x; // keep variable in an alloca
}
