//@ only-riscv32cheriot-unknown-cheriotrtos

// A simple example to verify that we generate the correct LLVM IR for the `cheriot_mmio` attribute.

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
}

#[no_mangle]
pub unsafe fn test() -> bool {
    // CHECK: %_1 = load i32, ptr addrspace(200) @UART0, align 4, !noundef !2
    // CHECK: %_0 = icmp eq i32 %_1, 0
    // CHECK: ret i1 %_0
    unsafe { UART0.field == 0 }
}

// CHECK: attributes #0 = { "cheriot_global_cap_import"="mem,uart,R----" }
