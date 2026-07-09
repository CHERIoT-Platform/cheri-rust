//@ only-riscv32cheriot-unknown-cheriotrtos

// Ensure we emit the correct calling conventions when defining and calling
// functions with the "cheriot-compartment-call" and "cheriot-compartment-callee"
// ABIs. It is not clear yet how these will be used, and they are subject to change.

#![no_std]
#![feature(abi_cheriot_compartment_call)]
#![crate_type = "lib"]

extern "cheriot-compartment-call" {
    fn compartment_call() -> ();
}

extern "cheriot-compartment-callee" {
    fn compartment_callee() -> ();
}

#[unsafe(no_mangle)]
pub fn do_compartment_calls() -> () {
    unsafe {
        // CHECK: call cheriot_compartmentcallcc {{.*}} @compartment_call()
        compartment_call();
        // CHECK: call cheriot_compartmentcalleecc {{.*}} @compartment_callee()
        compartment_callee();
    }
}

// CHECK: define dso_local cheriot_compartmentcallcc void @define_compartment_call()
#[unsafe(no_mangle)]
extern "cheriot-compartment-call" fn define_compartment_call() {}

// CHECK: define dso_local cheriot_compartmentcalleecc void @define_compartment_callee()
#[unsafe(no_mangle)]
extern "cheriot-compartment-callee" fn define_compartment_callee() {}
