#![no_std]
#![feature(abi_cheriot_library_call)]

extern crate alloc;
extern crate cheriot;

extern "cheriot-library-call" {
    fn __atomic_load_4(ptr: *const u32) -> u32;
}

#[no_mangle]
extern "C" fn test_libcalls() -> i32 {
    let x = 42;

    unsafe {
        assert_eq!(__atomic_load_4(&x), 42);
    }

    0
}
