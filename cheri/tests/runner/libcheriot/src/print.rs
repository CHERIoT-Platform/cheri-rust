//! A bare-bones implementation of `println!` and friends.

extern "C" {
    pub fn __rust_test_print_str(v: *const core::ffi::c_char);
    pub fn __rust_test_eprint_str(v: *const core::ffi::c_char);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    let str = alloc::string::ToString::to_string(&args);
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe {
        __rust_test_print_str(str.as_ptr());
    }

    drop(str);
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::print::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _eprint(args: core::fmt::Arguments) {
    let str = alloc::string::ToString::to_string(&args);
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe {
        __rust_test_eprint_str(str.as_ptr());
    }

    drop(str);
}

pub use eprint;
pub use eprintln;
pub use print;
pub use println;
