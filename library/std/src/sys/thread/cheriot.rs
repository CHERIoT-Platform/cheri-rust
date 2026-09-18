unsafe extern "C" {
    fn thread_id_get_wrapper() -> u16;
}

pub fn current_os_id() -> Option<u64> {
    Some(unsafe { thread_id_get_wrapper() as u64 })
}
