//@ add-minicore
//@ needs-llvm-components: riscv
//@ compile-flags: --target=riscv32cheriot-unknown-cheriotrtos

// This tests the errors when using the `cheri_compartment` attribute.

#![feature(no_core, lang_items, cheriot_attributes)]
#![no_core]

extern crate minicore;
use minicore::*;

// It doesn't make sense to mark a type definition with the compartment attribute,
// as only functions are visible from other compartments.
#[cheri_compartment = "comp1"] //~ ERROR the `cheri_compartment` attribute cannot be used on structs
pub struct Hello;

// At this stage it also doesn't make sense to have effectively visible functions
// marked as public. Exporting a function as a compartment entry-point is done
// with the crate-level cheri-compartment attribute, because mixed compartment exports
// and crate exports aren't supported.
#[cheri_compartment = "comp1"] //~ ERROR the `cheri_compartment` attribute cannot be used on functions
pub fn my_entry_point() {}

#[cheri_compartment = "comp1"]
unsafe extern "C" {

    // This function is already part of an extern block marked
    // with a cheri_compartment attribute. It is invalid to
    // add a new cheri_compartment attribute here.
    #[cheri_compartment = "comp2"] //~ ERROR
    pub fn comp2_entry_point();

    // It doesn't make sense to mark static globals as belonging
    // to a compartment.
    static mut MY_STATIC: i32; //~ ERROR invalid item in a extern block with the `#[cheri_compartment]` attribute
}
