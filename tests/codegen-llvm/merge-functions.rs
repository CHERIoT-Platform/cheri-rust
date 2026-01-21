//@ revisions: O Os
//@[Os] compile-flags: -Copt-level=s
//@[O] compile-flags: -Copt-level=3
//@ ignore-riscv32cheriot-unknown-cheriotrtos See CHERIoT-Platform/cheri-rust/issues/87

#![crate_type = "lib"]
#![no_std]

// CHECK: @func{{2|1}} = {{.*}}alias{{.*}}@func{{1|2}}

#[no_mangle]
pub fn func1(c: char) -> bool {
    c == 's' || c == 'm' || c == 'h' || c == 'd' || c == 'w'
}

#[no_mangle]
pub fn func2(c: char) -> bool {
    matches!(c, 's' | 'm' | 'h' | 'd' | 'w')
}
