#define MALLOC_QUOTA 0x100000

#include <allocator.h>
#include <compartment.h>
#include <debug.hh>
#include <priv/riscv.h>
#include <simulator.h>
#include <unwind.h>

using Debug = ConditionalDebug<true, "Test Runner">;

// Display information about a CHERI fault, then either unwind or exit.
// Aborts from Rust should trigger an invalid instruction which is caught here.
extern "C" ErrorRecoveryBehaviour
compartment_error_handler(ErrorState *frame, size_t mcause, size_t mtval) {
  if (mcause == priv::MCAUSE_CHERI) {
    // Note: handle CZR differently as `get_register_value` will return a
    // nullptr which we cannot dereference.

    auto [exceptionCode, registerNumber] = CHERI::extract_cheri_mtval(mtval);

    Debug::log("{} error at {} (return address: {}), with capability register "
               "{}: {}",
               exceptionCode, frame->pcc,
               frame->get_register_value<CHERI::RegisterNumber::CRA>(),
               registerNumber,
               registerNumber == CHERI::RegisterNumber::CZR
                   ? nullptr
                   : *frame->get_register_value(registerNumber));
  }

  // Calling cleanup_unwind without a registered handler is dangerous,
  // and if we don't have a handler then we want to ensure we exit with
  // a failure status (returning ForceUnwind will exit with success).
  struct CleanupList **head = cleanup_list_head();
  bool has_cleanup_handler = *head != nullptr;

  if (has_cleanup_handler) {
    Debug::log("Error, unwinding...");
    cleanup_unwind();
  } else {
    Debug::log("Error, exiting...");
    simulation_exit(1);
  }

  // if either call above fails (e.g. because we are not in a simulator or
  // there is no stack space available) fallback to forcibly unwinding
  return ErrorRecoveryBehaviour::ForceUnwind;
}

extern "C" void cheriot_print(char *s) { printf("%s", s); }

extern "C" void *cheriot_alloc(size_t size, size_t align) {
  Timeout timeout{5};
  void *ret = heap_allocate(&timeout, MALLOC_CAPABILITY, size);
  Debug::Invariant(CHERI::Capability{ret}.is_valid(),
                   "Allocation is invalid, got pointer: {} -- {}", ret,
                   (int)ret);
  return ret;
}

extern "C" void cheriot_free(void *ptr) { heap_free(MALLOC_CAPABILITY, ptr); }

// Re-export because `cleanup_list_head` is marked as `__always_inline static
// inline`.
extern "C" struct CleanupList **get_cleanup_list_head() {
  return cleanup_list_head();
}

extern "C" int rust_main();

int __cheri_compartment("test_runner") run() {
  int status = rust_main();
  simulation_exit(status);
}
