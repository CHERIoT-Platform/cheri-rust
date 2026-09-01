//@ only-riscv32cheriot-unknown-cheriotrtos

// A simple example to verify that we generate the correct LLVM IR
// for the `cheriot_mmio` and `cheriot_shared_object` attributes.

#![no_std]
#![feature(cheriot_attributes)]
#![crate_type = "lib"]

#[repr(C)]
pub struct Uart {
    field: u32,
}

unsafe extern "Rust" {
    // CHECK: @UART0 = external dso_local local_unnamed_addr addrspace(200) global %Uart #0
    #[cheriot_mmio(name = "uart", permissions = "R")]
    pub static UART0: Uart;

    // CHECK: @SHARED_OBJ = external dso_local local_unnamed_addr addrspace(200) global i32 #1
    #[cheriot_shared_object(name = "my_shared_object", permissions = "R")]
    pub static SHARED_OBJ: i32;
}

#[no_mangle]
pub unsafe fn test() -> bool {
    // CHECK: %_1 = load i32, ptr addrspace(200) @UART0, align 4, !noundef !{{[0-9]+}}
    // CHECK: %0 = icmp eq i32 %_1, 0
    // CHECK: %_3 = load i32, ptr addrspace(200) @SHARED_OBJ, align 4
    // CHECK: %1 = icmp eq i32 %_3, 0
    // CHECK: %_0.sroa.0.0.off0 = select i1 %0, i1 %1, i1 false
    // CHECK: ret i1 %_0.sroa.0.0.off0
    unsafe { UART0.field == 0 && SHARED_OBJ == 0 }
}

// CHECK: attributes #0 = { "cheriot_global_cap_import"="mem,uart,R----" }
// CHECK: attributes #1 = { "cheriot_global_cap_import"="cheriot_shared_object,my_shared_object,R----" }
