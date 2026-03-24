use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn cheriot_print(str: *const c_char);

    pub fn cheriot_alloc(size: u32, align: u32) -> *mut c_void;

    pub fn cheriot_free(ptr: *mut c_void);
}
