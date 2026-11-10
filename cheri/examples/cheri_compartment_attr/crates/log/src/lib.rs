#![no_std]
#![feature(cheriot_attributes)]
#![cheri_compartment = "log4rust"]

use cheriot::*;

/// Log a new entry.
pub fn record(key: &str, value: &str) {
    println!("[log] {key} -> {value}");
}
