unsafe extern "C" {
    fn cheriot_print_str(v: *const core::ffi::c_char);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (print_args(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn print_args(args: core::fmt::Arguments<'_>) {
    let str = alloc::string::ToString::to_string(&args);
    let str = alloc::ffi::CString::new(str).unwrap();

    unsafe {
        cheriot_print_str(str.as_ptr());
    }

    drop(str);
}

pub use {print, println};
