use crate::print::*;

#[repr(C)]
#[derive(Default)]
struct __jmp_buf {
    __cs0: *const (),
    __cs1: *const (),
    __csp: *const (),
    __cra: *const (),
}

#[repr(C)]
#[derive(Default)]
struct CleanupList {
    /// Next pointer.
    next: *mut CleanupList,
    /// Jump buffer to return to.
    env: __jmp_buf,
}

unsafe extern "C" {
    fn get_cleanup_list_head() -> *mut *mut CleanupList;
    fn setjmp(env: *const __jmp_buf) -> core::ffi::c_int;
    fn do_cleanup_unwind() -> !;
}

// not sure how best to capture this
static mut PANIC_MESSAGE: &'static str = "";

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        PANIC_MESSAGE = alloc::format!("{info}").leak();
        do_cleanup_unwind();
    }
}

#[inline(never)]
pub fn try_run<F, T>(run: F, should_panic: bool) -> Result<T, ()>
where
    F: FnOnce() -> T,
{
    unsafe {
        let mut cleanup_list_entry = CleanupList::default();
        let head = get_cleanup_list_head();
        cleanup_list_entry.next = *head;
        *head = &mut cleanup_list_entry;
        if core::ptr::read_volatile(&setjmp(&cleanup_list_entry.env)) == 0 {
            let result = run();
            *head = cleanup_list_entry.next;
            Ok(result)
        } else {
            *head = cleanup_list_entry.next;
            if !should_panic {
                let msg = core::ptr::read(&raw const PANIC_MESSAGE);
                println!("{}", msg);
                PANIC_MESSAGE = "";
            }
            Err(())
        }
    }
}
