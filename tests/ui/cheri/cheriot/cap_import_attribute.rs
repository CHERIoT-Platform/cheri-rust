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

#[cheriot_mmio(name = "uart", permissions = "R")] //~ ERROR the `cheriot_mmio` attribute cannot be used on statics
pub static UART_NOEXT: i8 = 0;

#[cheriot_shared_object(name = "my_so", permissions = "R")] //~ ERROR the `cheriot_shared_object` attribute cannot be used on statics
pub static SHARED_OBJECT_NOEXT: i8 = 0;

#[cheriot_shared_object(name = "my_so", permissions = "R")]  //~ ERROR multiple CHERIoT capability import attributes used on the same item
//~^ ERROR  the `cheriot_shared_object` attribute cannot be used on static
#[cheriot_mmio(name = "my_so", permissions = "R")] //~ ERROR multiple CHERIoT capability import attributes used on the same item
//~^ ERROR  the `cheriot_mmio` attribute cannot be used on static
pub static TWO_ATTRS: i8 = 0;

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


    #[cheriot_shared_object(name = "my_so", permissions = "")] //~ ERROR permissions `` are invalid as they don't contain either `R` (read) or `W` (write)
    pub static SHARED_OBJ_EMPTY: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "c")] //~ ERROR permissions `c` are invalid as they don't contain either `R` (read) or `W` (write)
    pub static SHARED_OBJ_C: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "m")] //~ ERROR permissions `m` are invalid as they contain load mut (`m`) but don't contain both read (`R`) and cap (`c`)
    pub static SHARED_OBJ_M: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "Wm")] //~ ERROR permissions `Wm` are invalid as they contain load mut (`m`) but don't contain both read (`R`) and cap (`c`)
    pub static SHARED_OBJ_WM: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "mc")] //~ ERROR permissions `mc` are invalid as they contain load mut (`m`) but don't contain both read (`R`) and cap (`c`)
    pub static SHARED_OBJ_MC: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "g")] //~ ERROR permissions `g` are invalid as they contain load global (`g`) but don't contain both read (`R`) and cap (`c`)
    pub static SHARED_OBJ_G: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "Wg")] //~ ERROR permissions `Wg` are invalid as they contain load global (`g`) but don't contain both read (`R`) and cap (`c`)
    pub static SHARED_OBJ_WG: i32;

    #[cheriot_shared_object(name = "my_so", permissions = "gc")] //~ ERROR permissions `gc` are invalid as they contain load global (`g`) but don't contain both read (`R`) and cap (`c`)
    pub static SHARED_OBJ_GC: i32;

}

pub fn main() {}
