//! Our reproduction of the `print!` and `println!` macros.

extern crate alloc;

unsafe extern "C" {
    // For now the implementation of the actual printing is delegated to the C shim.
    pub fn cheriot_print_str(v: *const core::ffi::c_char);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (cheriot::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let str = alloc::string::ToString::to_string(&args);
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe {
        cheriot_print_str(str.as_ptr());
    }

    drop(str);
}

#[allow(unused)]
pub use {print, println};
