#pragma once

#include <cdefs.h>
#include <stdint.h>

// libm
CHERIOT_DECLARE_STANDARD_LIBCALL(sqrtf, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(expf, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(exp2f, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(logf, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(log2f, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(log10f, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(sqrt, double, double)
CHERIOT_DECLARE_STANDARD_LIBCALL(exp, double, double)
CHERIOT_DECLARE_STANDARD_LIBCALL(exp2, double, double)
CHERIOT_DECLARE_STANDARD_LIBCALL(log, double, double)
CHERIOT_DECLARE_STANDARD_LIBCALL(log2, double, double)
CHERIOT_DECLARE_STANDARD_LIBCALL(log10, double, double)
CHERIOT_DECLARE_STANDARD_LIBCALL(fminimum_numf, float, float, float)
CHERIOT_DECLARE_STANDARD_LIBCALL(fmaximum_numf, float, float, float)

CHERIOT_DECLARE_STANDARD_LIBCALL(ceil, double, double)
