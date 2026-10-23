//! A simple crate. The objective is to be familiar with the xmake rules to build crates
//! and pass flags to rustc/cargo.

#![no_std]
#![feature(cheriot_attributes)]
#![cheri_compartment = "my_app"]

#[cfg(not(target_family = "cheriot"))]
compile_error!("This crate can only compile to CHERIoT!");

#[allow(unused)]
use cheriot::*;

/// Our entry-point. Simulates getting username and password from the user.
#[unsafe(export_name = "_Z9call_rustv")]
pub fn call_rust() -> core::ffi::c_int {
    log::record("hello", "from rust");
    core::ffi::c_int::from(0)
}
