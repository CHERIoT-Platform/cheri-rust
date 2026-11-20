unsafe extern "cheriot-library-call" {
    #[link_name = "_Z13thread_id_getv"]
    fn thread_id_get() -> u16;
}

pub fn current_os_id() -> Option<u64> {
    Some(unsafe { thread_id_get() as u64 })
}
