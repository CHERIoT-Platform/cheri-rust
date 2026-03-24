#define MALLOC_QUOTA 0x100000

#include "fail-simulator-on-error.h"
#include <allocator.h>
#include <debug.hh>

using Test = ConditionalDebug<true, "Test Runner">;

extern "C" void cheriot_print(char *s) { printf("%s", s); }

extern "C" void *cheriot_alloc(size_t size, size_t align) {
  Timeout timeout{5};
  void *ret = heap_allocate(&timeout, MALLOC_CAPABILITY, size);
  Test::Invariant(CHERI::Capability{ret}.is_valid(),
                  "Allocation is invalid, got pointer: {} -- {}", ret,
                  (int)ret);
  return ret;
}

extern "C" void cheriot_free(void *ptr) { heap_free(MALLOC_CAPABILITY, ptr); }

extern "C" int rust_main();

int __cheri_compartment("test_runner") run() {
  rust_main();
  simulation_exit(0);
}
