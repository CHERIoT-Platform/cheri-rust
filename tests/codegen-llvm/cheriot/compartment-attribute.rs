//@ only-riscv32cheriot-unknown-cheriotrtos
//@ compile-flags: -Copt-level=0

// Objectives:
//
// 1. Ensure that when a crate has the `cheri_compartment` attribute the
//    functions visible from other crates (and only those) have the correct
//    CC.
//
// 2. Verify that the `extern "C"` functions with the `cheri_compartment`
//    attribute are correctly generated (i.e. they have the correct CC)
//
// Note that the `#[no_mangle]` attributes here don't have a role, they are
// just to force rustc keep the functions in the LLVM IR. Also note that
// the order of definitions and declarations here follows the order of what
// will appear in the LLVM IR.

#![no_std]
#![feature(cheriot_attributes)]
#![crate_type = "lib"]
#![cheri_compartment = "comp1"]

// Public, should be visibile from other crates too: should be marked with the
// compartment CC. Notice that there is no annotation on the function; the fact
// that it belongs to a compartment is due to the crate-wide annotation at the
// top.
#[no_mangle]
// CHECK:  define dso_local cheriot_compartmentcalleecc void @public() unnamed_addr addrspace(200) #0
pub fn public() {
    unsafe {
        // Other compartment; needs cheriot_compartmentcallcc.
        // CHECK: call cheriot_compartmentcallcc addrspace(200) void @comp2_entrypoint()
        comp2_entrypoint();

        // Same as above: it's here just to check that the attribute works on blocks.
        // CHECK: call cheriot_compartmentcallcc addrspace(200) void @comp3_entrypoint()
        comp3_entrypoint();

        // This one is in the same compartment: should be cheriot_compartmentcalleecc.
        // CHECK: call cheriot_compartmentcalleecc addrspace(200) void @other_comp1_entrypoint()
        other_comp1_entrypoint();
    }

    // Public within the `prv` module but not exposed externally; normal CC.
    // CHECK: call addrspace(200) void @_RNvNtCsggRB0ItIQTK_21compartment_attribute3prv17not_an_entrypoint()
    prv::not_an_entrypoint();
}

mod prv {
    // Public, but shouldn't be marked with the compartment CC, since it's
    // effectively visible only in the current crate.
    // CHECK: define internal void @_RNvNtCsggRB0ItIQTK_21compartment_attribute3prv17not_an_entrypoint() unnamed_addr addrspace(200) #0
    pub fn not_an_entrypoint() {}
}

// Not public: no compartment CC.
#[no_mangle]
// CHECK: define dso_local void @not_public() unnamed_addr addrspace(200) #0
fn not_public() {
    prv::not_an_entrypoint();
}

unsafe extern "C" {
    // Notice that real applications here most likely would want to use the
    // `#[link_name]` attribute to use the Itanium ABI-mangled name of the
    // export.
    #[cheri_compartment = "comp2"]
    // CHECK: declare dso_local cheriot_compartmentcallcc void @comp2_entrypoint() unnamed_addr addrspace(200) #1
    pub fn comp2_entrypoint();
}

// Compartment annotations should work on extern blocks too.
#[cheri_compartment = "comp3"]
unsafe extern "C" {
    // CHECK: declare dso_local cheriot_compartmentcallcc void @comp3_entrypoint() unnamed_addr addrspace(200) #2
    pub fn comp3_entrypoint();
}

// Compartment annotations should work on extern blocks too.
#[cheri_compartment = "comp1"]
unsafe extern "C" {
    // CHECK: declare dso_local cheriot_compartmentcalleecc void @other_comp1_entrypoint() unnamed_addr addrspace(200) #0
    pub fn other_comp1_entrypoint();
}

// CHECK: attributes #0 = { {{.*}} "cheri-compartment"="comp1"
// CHECK: attributes #1 = { {{.*}} "cheri-compartment"="comp2"
// CHECK: attributes #2 = { {{.*}} "cheri-compartment"="comp3"
