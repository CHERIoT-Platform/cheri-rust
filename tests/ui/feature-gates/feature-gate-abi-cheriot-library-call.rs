//@ revisions: HOST CHERIOT
//@ add-minicore
//@ compile-flags: --crate-type=rlib
//@[CHERIOT] needs-llvm-components: riscv
//@[CHERIOT] compile-flags: --target=riscv32cheriot-unknown-cheriotrtos
//@ ignore-backends: gcc
#![no_core]
#![feature(no_core, lang_items)]

// Test that the "cheriot-library-call" ABI is feature-gated on CHERIoT,
// and not available on the host target.

extern crate minicore;
use minicore::*;

extern "cheriot-library-call" fn f() {} //~ ERROR "cheriot-library-call" ABI is experimental and subject to change
//[HOST]~^ ERROR is not a supported ABI
