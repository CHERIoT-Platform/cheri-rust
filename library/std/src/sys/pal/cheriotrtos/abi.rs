use core::ffi::c_void;

unsafe extern "C" {
    pub fn cheriot_write(str: *const u8, len: usize) -> usize;
    pub fn cheriot_alloc(size: u32) -> *mut c_void;
    pub fn cheriot_free(ptr: *mut c_void);
}
