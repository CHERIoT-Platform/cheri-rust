//@ add-minicore
//@ needs-llvm-components: riscv
//@ compile-flags: --target=riscv32cheriot-unknown-cheriotrtos

// This tests the errors when using the `cheri_compartment` attribute.

#![feature(no_core, lang_items, cheriot_attributes, abi_cheriot_library_call)]
#![no_core]
#![cheri_compartment = "comp0"]

extern crate minicore;
use minicore::*;

// It doesn't make sense to mark a type definition with the compartment attribute,
// as only functions are visible from other compartments.
#[cheri_compartment = "comp1"] //~ ERROR the `cheri_compartment` attribute cannot be used on structs
pub struct Hello;

#[cheri_compartment = "comp1"] //~ ERROR the `cheri_compartment` attribute cannot be used on statics
pub static mut HELLO: i32 = 0; //~ ERROR invalid public item in compartment

pub static mut HELLO2: i32 = 0; //~ ERROR invalid public item in compartment
pub const  HELLO3: i32 = 0; //~ ERROR invalid public item in compartment

// We don't allow an empty compartment name.
#[cheri_compartment = ""] //~ ERROR the compartment name in `#[cheri_compartment = "..."] cannot be empty
unsafe extern "C" {
    fn missing_compartment_name();
}

// At this stage it also doesn't make sense to have effectively visible functions
// marked as public. Exporting a function as a compartment entry-point is done
// with the crate-level cheri-compartment attribute, because mixed compartment exports
// and crate exports aren't supported.
#[cheri_compartment = "comp1"] //~ ERROR the `cheri_compartment` attribute cannot be used on functions
pub fn entry_point0() {}


pub extern "cheriot-library-call" fn entry_point1() {} //~  ERROR incompatible ABI for compartment entry-point declaration

mod prv {
    // Fine: not really public.
    pub extern "cheriot-library-call" fn prv_entry_point0() {}

    // Not fine: re-exported later.
    pub extern "cheriot-library-call" fn prv_entry_point1() {} //~  ERROR incompatible ABI for compartment entry-point declaration
}

pub use prv::prv_entry_point1;

// This one should not fail because it is private, although it doesn't make sense
// for a compartment to export a library entry-point.
extern "cheriot-library-call" fn fake_library_entry_point() {}

// No polymorphism in compartment entry-points
pub fn entry_point2<T>(arg: T) -> T { arg } //~ ERROR use of generics in compartment entry-point
pub fn entry_point3(_arg: impl Copy) -> usize { 0 } //~ ERROR use of generics in compartment entry-point
pub fn entry_point4(_arg: impl Copy) -> impl Copy { 0 } //~ ERROR use of generics in compartment entry-point

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



#[cheri_compartment = "comp1"]
unsafe extern "cheriot-library-call" { //~  ERROR incompatible ABI for compartment entry-point declaration
        fn comp1_lib_entry_point();
}
