//@ add-minicore
//@ needs-llvm-components: riscv
//@ compile-flags: --target=riscv32cheriot-unknown-cheriotrtos

// This tests the errors when using the `cheriot_mmio` attribute.

#![feature(no_core, lang_items, cheriot_attributes)]
#![no_core]

extern crate minicore;
use minicore::*;

#[repr(C)]
pub struct Uart {
    field: u32,
}

#[cheriot_mmio(name = "uart", permissions = "R")] //~ ERROR `#[cheriot_mmio]` attribute cannot be used on statics
pub static UART_NOEXT: i8 = 0;

unsafe extern "Rust" {

    #[cheriot_mmio(name = "uart", permissions = "")] //~ ERROR permissions `` are invalid as they don't contain either `R` (read) or `W` (write)
    pub static UART_EMPTY: Uart;

    #[cheriot_mmio(name = "uart", permissions = "c")] //~ ERROR permissions `c` are invalid as they don't contain either `R` (read) or `W` (write)
    pub static UART_C: Uart;

    #[cheriot_mmio(name = "uart", permissions = "m")] //~ ERROR permissions `m` are invalid as they contain load mut (`m`) but don't contain both read (`R`) and cap (`c`)
    pub static UART_M: Uart;

    #[cheriot_mmio(name = "uart", permissions = "Wm")] //~ ERROR permissions `Wm` are invalid as they contain load mut (`m`) but don't contain both read (`R`) and cap (`c`)
    pub static UART_WM: Uart;

    #[cheriot_mmio(name = "uart", permissions = "mc")] //~ ERROR permissions `mc` are invalid as they contain load mut (`m`) but don't contain both read (`R`) and cap (`c`)
    pub static UART_MC: Uart;

    #[cheriot_mmio(name = "uart", permissions = "g")] //~ ERROR permissions `g` are invalid as they contain load global (`g`) but don't contain both read (`R`) and cap (`c`)
    pub static UART_G: Uart;

    #[cheriot_mmio(name = "uart", permissions = "Wg")] //~ ERROR permissions `Wg` are invalid as they contain load global (`g`) but don't contain both read (`R`) and cap (`c`)
    pub static UART_WG: Uart;

    #[cheriot_mmio(name = "uart", permissions = "gc")] //~ ERROR permissions `gc` are invalid as they contain load global (`g`) but don't contain both read (`R`) and cap (`c`)
    pub static UART_GC: Uart;
}

pub fn main() {}
