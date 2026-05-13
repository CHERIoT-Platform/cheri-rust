/// See `cheriot-rtos/sdk/include/unwind.h`.
///
/// We should assess the safety and appropriateness of this error handling
/// strategy. In this context (running short-lived tests) it is probably OK.
///
/// Note that `std/panicking` thinks that we never recover from a first panic,
/// which can have an effect on the behaviour of future panics. We might resolve
/// this by making our handling compatible with `catch_unwind` (which is still
/// callable with panic=abort).
///
/// With `std` we are not able to register our own `panic_handler`, which is why
/// we use the `panic_hook` to capture the panic message. If our allocator fails
/// we may not be able to recover correctly.
///
/// We should investigate the viability of supporting either the `panic_unwind` runtime
/// or supporting `std/sys/thread` as possible alternative approaches.
///
/// See also `library/panic_abort`, `library/panic_unwind` and `library/std/panicking`.
use std::cell::{Cell, RefCell};

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
    next: *mut CleanupList,
    env: __jmp_buf,
}

unsafe extern "C" {
    fn get_cleanup_list_head() -> *mut *mut CleanupList;
    fn setjmp(env: *const __jmp_buf) -> core::ffi::c_int;
}

thread_local! {
    static PANIC_MESSAGE: RefCell<String> = RefCell::new(String::new());
    static PANIC_EXPECT: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn set_panic_hook() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !PANIC_EXPECT.get() {
            prev(info);
            return;
        }

        let msg = match info.payload_as_str() {
            Some(s) => s.to_string(),
            None => info.to_string(),
        };

        PANIC_MESSAGE.with_borrow_mut(|m| *m = msg);
    }));
}

#[inline(never)]
pub(crate) fn try_run<F: FnOnce() -> R, R>(run: F) -> Result<R, String> {
    unsafe {
        let mut cleanup_list_entry = CleanupList::default();
        let head = get_cleanup_list_head();
        cleanup_list_entry.next = *head;
        *head = &mut cleanup_list_entry;
        // Rust will take for granted that a function only returns once
        if core::ptr::read_volatile(&setjmp(&cleanup_list_entry.env)) == 0 {
            PANIC_EXPECT.set(true);
            let result = run();
            *head = cleanup_list_entry.next;
            PANIC_EXPECT.set(false);
            Ok(result)
        } else {
            *head = cleanup_list_entry.next;
            PANIC_EXPECT.set(false);
            Err(PANIC_MESSAGE.with_borrow_mut(|m| std::mem::take(m)))
        }
    }
}
