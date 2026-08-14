//@ add-minicore

#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

#[repr(C)]
pub struct Uart {
    field: u32,
}

unsafe extern "Rust" {
    #[cheriot_mmio(name = "uart", permissions = "R")] //~ ERROR  the `#[cheriot_mmio]` attribute is an experimental feature
    pub static UART0: Uart; //~^ ERROR attribute `cheriot_mmio` can be used on CHERIoT targets only
}

pub fn main() {}
