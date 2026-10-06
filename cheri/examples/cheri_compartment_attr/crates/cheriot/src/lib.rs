//! Support CHERIoT libraries.
//! These will, in the future, be part of an BSP maintained by SCI.

//! This module implements the panic handler and the allocator, which Rust expects from us.
//!
//! It also implements some utility functions to print to the UART.

#![no_std]

#[cfg(not(target_family = "cheriot"))]
compile_error!("This crate can only compile to CHERIoT!");

extern crate alloc;

mod cheriot_alloc;
mod print;

#[allow(unused_imports)]
pub use print::*;

/// Our panic handler. Does nothing: it just loops.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
