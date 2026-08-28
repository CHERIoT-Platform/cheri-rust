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

extern "C" size_t cheriot_write(uint8_t *buf, size_t len) {
  auto uart = MMIO_CAPABILITY(Uart, uart);
  size_t i = 0;
  while (i < len && uart->can_write()) {
    uart->data = buf[i++];
  }
  return i;
}

extern "C" void *cheriot_alloc(size_t size) {
  // wait for revocation, don't wait for free
  void *ret = heap_allocate(TimeoutWaitForever, MALLOC_CAPABILITY, size,
                            AllocateWaitRevocationNeeded);
  if (!CHERI::Capability{ret}.is_valid()) {
    Debug::log("Trying to allocate {} bytes", (int)size);
    Debug::log("Allocator error on alloc: {} ({})", (int)ret, ret);
    return nullptr;
  }
  return ret;
}

extern "C" void cheriot_free(void *ptr) {
  int ret = heap_free(MALLOC_CAPABILITY, ptr);
  Debug::Invariant(ret >= 0, "Allocator error on free: {}", ret);
}

extern "C" void cheriot_quarantine_flush() {
  int ret = heap_quarantine_empty();
  Debug::Invariant(ret >= 0, "Allocator error on flush: {}", ret);
}

// Re-export because `cleanup_list_head` is marked as `__always_inline static
// inline`.
extern "C" struct CleanupList **get_cleanup_list_head() {
  return cleanup_list_head();
}

extern "C" int rust_main();

// We probably want `Allocator::check_gm` but this is not exposed. We want to
// ensure that the allocator is working, and print the size of the heap which
// is available to us. `heap_available` returns an error unless the allocator
// has been initialised (through a call to `check_gm`), which is the primary
// reason we make an allocation here, though it adds some extra useful checks.
void startup_check_allocator() {
  void *ptr = heap_allocate(TimeoutNoWait, MALLOC_CAPABILITY, 16);
  Debug::Invariant(CHERI::Capability{ptr}.is_valid(),
                   "Allocator error at startup: {}", (int)ptr);

  heap_free(MALLOC_CAPABILITY, ptr);
  heap_quarantine_empty();

  int available = (int)heap_available();
  Debug::Invariant(available >= 0, "Allocator error at startup: {}", available);

  Debug::log("Heap available: {} bytes", available);
}

int __cheri_compartment("test_runner") run() {
  startup_check_allocator();

  int status = rust_main();
  simulation_exit(status);
}
