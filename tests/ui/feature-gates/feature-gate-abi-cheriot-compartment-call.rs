//@ revisions: HOST CHERIOT
//@ add-minicore
//@ compile-flags: --crate-type=rlib
//@[CHERIOT] needs-llvm-components: riscv
//@[CHERIOT] compile-flags: --target=riscv32cheriot-unknown-cheriotrtos
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items)]

// Test that the "cheriot-compartment-call" and "cheriot-compartment-callee" ABIs
// are feature-gated on CHERIoT, and not available on the host target.

extern crate minicore;
use minicore::*;

extern "cheriot-compartment-call" fn f1() {} //~ ERROR "cheriot-compartment-call" ABI is experimental and subject to change
//[HOST]~^ ERROR is not a supported ABI

extern "cheriot-compartment-callee" fn f2() {} //~ ERROR "cheriot-compartment-callee" ABI is experimental and subject to change
//[HOST]~^ ERROR is not a supported ABI
