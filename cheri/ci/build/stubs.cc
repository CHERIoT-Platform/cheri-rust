// Turn some missing libcalls into runtime errors

#include <stdint.h>
#include <debug.hh>

using Debug = ConditionalDebug<true, "Test Runner">;

#define STUB_LIBCALL(name, ret, ...) \
    ret name(__VA_ARGS__) { \
        Debug::Invariant(false, "Missing libcall: " #name); \
        __builtin_trap(); \
    }

// softfloat
STUB_LIBCALL(__extendhfsf2, float, uint16_t)
STUB_LIBCALL(__truncsfhf2, uint16_t, float)

// libm
STUB_LIBCALL(sqrtf, float, float)
STUB_LIBCALL(expf, float, float)
STUB_LIBCALL(exp2f, float, float)
STUB_LIBCALL(logf, float, float)
STUB_LIBCALL(log2f, float, float)
STUB_LIBCALL(log10f, float, float)
STUB_LIBCALL(sqrt, double, double)
STUB_LIBCALL(exp, double, double)
STUB_LIBCALL(exp2, double, double)
STUB_LIBCALL(log, double, double)
STUB_LIBCALL(log2, double, double)
STUB_LIBCALL(log10, double, double)
STUB_LIBCALL(fminimum_numf, float, float, float)
STUB_LIBCALL(fmaximum_numf, float, float, float)
