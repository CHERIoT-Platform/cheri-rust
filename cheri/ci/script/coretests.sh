#!/bin/bash

# Run all coretest modules, intended for use in CI.
#
# Due to target limitations, we have divided coretests into subsets which
# are gated by feature flags containing their module name. This script
# iterates over all modules, calling `./cheri/ci/script/coretest.sh` which
# invokes cargo test. The script collects the results and prints the
# sum of total/pass/fail/ignore. See prerequisites in
#   `./cheri/ci/script/coretest.sh`.

features=(
    "alloc"
    "any"
    "array"
    "ascii"
    "ascii_char"
    "asserting"
    "async_iter"
    "atomic"
    "bool"
    "bstr"
    "cell"
    "char"
    "clone"
    "cmp"
    "const_ptr"
    "convert"
    "ffi"
    "fmt"
    "fmt_builders"
    "fmt_float"
    "fmt_num"
    "future"
    "hash"
    "hint"
    "index"
    "intrinsics"
    "io"
    "iter"
    "iter_adapters"
    "lazy"
    "macros"
    "manually_drop"
    "mem"
    "net"
    "nonzero"
    "num"
    "num_rest"
    "ops"
    "option"
    "panic"
    "pattern"
    "pin"
    "pin_macro"
    "ptr"
    "result"
    "simd"
    "slice"
    "str"
    "str_lossy"
    "task"
    "time"
    "tuple"
    "unicode"
    "waker"
    "wtf8"
)

total_sum=0
ignored_sum=0
pass_sum=0

for feature in "${features[@]}"; do
    echo "[RUN] coretests/$feature..."

    output=$(./cheri/ci/script/coretest.sh "$feature" 2>&1)
    status=$?

    echo "$output"

    if [[ $status -ne 0 ]]; then
        echo "Simulator exited with failure"
        exit 1
    fi

    # e.g. [OK] total=6 ignored=0 pass=6
    results=$(echo "$output" | grep "^\[OK\] ")

    # The simulator can exit with success under certain error conditions
    if [[ -z "$results" ]]; then
        echo "Simulator exited prematurely"
        exit 1
    fi

    total=$(echo "$results" | sed 's/.*total=\([0-9]*\).*/\1/')
    ignored=$(echo "$results" | sed 's/.*ignored=\([0-9]*\).*/\1/')
    pass=$(echo "$results" | sed 's/.*pass=\([0-9]*\).*/\1/')

    total_sum=$((total_sum + total))
    ignored_sum=$((ignored_sum + ignored))
    pass_sum=$((pass_sum + pass))

done

echo "[ALL OK] total=$total_sum ignored=$ignored_sum pass=$pass_sum"
