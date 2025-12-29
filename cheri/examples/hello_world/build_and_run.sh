#!/bin/bash

CHERIOT_LLVM_PROJECT_PATH=${CHERIOT_SYSROOT_DIR:-"../../../build/host/llvm/"}
CHERIOT_RUSTC_PATH="../../../build/host/stage1/bin/rustc"

# Assumptions: you already have compiled the compiler. You also have cargo, git, xmake, a CHERIoT simulator and CHERIoT LLVM with clang.
set -x

# 1. Clone the RTOS.
git clone https://github.com/CHERIoT-Platform/cheriot-rtos.git --recursive --depth=1

# 2. Configure and run xmake.
xmake config -P . --sdk="$CHERIOT_LLVM_PROJECT_PATH" --rc="$CHERIOT_RUSTC_PATH" && xmake run
