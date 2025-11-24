#!/bin/bash

RUSTC_TOOLCHAIN_NAME="cheri"
CHERIOT_LLVM_PROJECT_PATH=${CHERIOT_SYSROOT_DIR:-"../../../build/host/llvm"}

# Assumptions: you already have compiled the compiler, and `cargo +$RUSTC_TOOLCHAIN_NAME` works. You also have git, xmake, a CHERIoT simulator and CHERIoT LLVM with clang.
set -x

# 1. Build the project.
cargo "+$RUSTC_TOOLCHAIN_NAME" build --target=riscv32cheriot-unknown-cheriotrtos --release

# 2. Clone the RTOS.
git clone https://github.com/xdoardo/cheriot-rtos.git --recursive --depth=1 --branch="fix-add-ldflags"

# 3. Configure and run xmake.
xmake config -P . --sdk="$CHERIOT_LLVM_PROJECT_PATH" && xmake run
