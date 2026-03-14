//@ compile-flags: -C no-prepopulate-passes
//

#![crate_type = "lib"]
#![no_std]

pub struct Bytes {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

// CHECK-LABEL: small_array_alignment
// The array is stored as i32, but its alignment is lower, go with 1 byte to avoid target
// dependent alignment
#[no_mangle]
pub fn small_array_alignment(x: &mut [i8; 4], y: [i8; 4]) {
    // CHECK: [[TMP:%.+]] = alloca [4 x i8], align 4
    // CHECK: %y = alloca [4 x i8], align 1
    // CHECK: store i32 %0, ptr[[ADDRSPACE]] [[TMP]]
    // CHECK: call[[ADDRSPACE]] void @llvm.memcpy.{{.*}}(ptr[[ADDRSPACE]] align 1 {{.+}}, ptr[[ADDRSPACE]] align 4 {{.+}}, i{{[0-9]+}} 4, i1 false)
    *x = y;
}

// CHECK-LABEL: small_struct_alignment
// The struct is stored as i32, but its alignment is lower, go with 1 byte to avoid target
// dependent alignment
#[no_mangle]
pub fn small_struct_alignment(x: &mut Bytes, y: Bytes) {
    // CHECK: [[TMP:%.+]] = alloca [4 x i8], align 4
    // CHECK: %y = alloca [4 x i8], align 1
    // CHECK: store i32 %0, ptr[[ADDRSPACE]] [[TMP]]
    // CHECK: call[[ADDRSPACE]] void @llvm.memcpy.{{.*}}(ptr[[ADDRSPACE]] align 1 {{.+}}, ptr[[ADDRSPACE]] align 4 {{.+}}, i{{[0-9]+}} 4, i1 false)
    *x = y;
}
