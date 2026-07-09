//@ only-riscv32cheriot-unknown-cheriotrtos

// Ensure we emit the correct calling conventions when defining and calling
// functions with the "cheriot-library-call" ABI.

#![no_std]
#![feature(abi_cheriot_library_call)]
#![crate_type = "lib"]

extern "cheriot-library-call" {
    fn __atomic_load_4(ptr: *const u32) -> u32;
}

pub fn do_libcall() -> bool {
    let x = 42;
    unsafe {
        // CHECK: call cheriot_librarycallcc {{.*}} @__atomic_load_4(
        __atomic_load_4(&x) == 42
    }
}

// CHECK: define dso_local cheriot_librarycallcc void @define_library_call()
#[unsafe(no_mangle)]
extern "cheriot-library-call" fn define_library_call() {}
