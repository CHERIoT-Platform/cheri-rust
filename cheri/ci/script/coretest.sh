#!/bin/bash

# Run a single module in coretests, intended for use in CI.
#
# We bypass `bootstrap` here as we need a level of control over the build
# and execution of tests due to current limitations. We set the linker and
# runner with cargo config flags because other approaches (config files, env
# vars) cannot handle our target name correctly.
#
# Prior to running this script, we should already have the compiler and std
# library built with e.g.
#   `x build compiler std --target riscv32cheriot-unknown-cheriotrtos.facade`
#
# Run from the top-level with the module name as the argument, e.g.
#   `./cheri/ci/script/coretest.sh alloc`

CARGO="./build/host/stage0/bin/cargo"
LINKER="./cheri/ci/script/linker.sh"
RUNNER="cheriot_sim"

if [[ -z "$1" ]]; then
    echo "Missing argument"
    exit 1
fi

RUSTC_SYSROOT="./build/host/stage1" \
RUSTC="./build/host/stage1/bin/rustc" \
RUSTC_BOOTSTRAP="1" \
RUSTFLAGS="-C embed-bitcode=yes \
    -C panic=abort \
    -C lto \
    -C codegen-units=1 \
    -C opt-level=3" \
$CARGO test \
    --manifest-path "./library/alloc/Cargo.toml" \
    --target "riscv32cheriot-unknown-cheriotrtos.facade" \
    --profile "release" \
    -p "coretests" \
    --tests \
    --features "compiler-builtins-mem coretests/test_$1" \
    -Zno-embed-metadata \
    --config "target.'cfg(target_abi = \"cheriot\")'.linker = '$LINKER'" \
    --config "target.'cfg(target_abi = \"cheriot\")'.runner = '$RUNNER'"
