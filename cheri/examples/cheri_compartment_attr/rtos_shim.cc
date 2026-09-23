#define MALLOC_QUOTA 0x100000

#include <allocator.h>
#include <debug.hh>

using Debug = ConditionalDebug<true, "Hello world compartment">;

/* Things that Rust expects from us */
extern "C" void cheriot_print_str(char *s)
{
	printf("%s", s);
}

extern "C" void *cheriot_alloc(size_t size)
{
	// debug_log("Trying to allocate {} bytes!", size);
	Timeout timeout{5};
	void   *ret = heap_allocate(&timeout, MALLOC_CAPABILITY, size);

	Debug::Invariant(CHERI::Capability{ret}.is_valid(),
	                 "Allocation is invalid, got pointer: {} -- {}",
	                 ret,
	                 (int)ret);
	return ret;
}

extern "C" void cheriot_free(void *ptr)
{
	free(ptr);
}
