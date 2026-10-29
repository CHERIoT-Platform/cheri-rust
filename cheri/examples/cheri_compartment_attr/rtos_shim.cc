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
    // wait for revocation, don't wait for free
    void *ret = heap_allocate(TimeoutWaitForever, MALLOC_CAPABILITY, size,
        AllocateWaitRevocationNeeded);

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
