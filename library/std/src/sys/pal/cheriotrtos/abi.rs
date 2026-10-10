unsafe extern "C" {
    pub fn cheriot_write(str: *const u8, len: usize) -> usize;

    pub fn cheriot_alloc(size: u32) -> *mut u8;

    pub fn cheriot_free(ptr: *mut u8);
}
