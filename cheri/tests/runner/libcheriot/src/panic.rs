extern "C" {
    pub fn __rust_test_panic(v: *const core::ffi::c_char) -> !;
}

/// The exported panic handler.
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    let str = alloc::format!("{info}");
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe { __rust_test_panic(str.as_ptr()) }
}
