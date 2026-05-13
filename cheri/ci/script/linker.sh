#!/bin/bash

# This script is intended to be passed to `rustc` as the linker for our target.
# This is a temporary solution and not designed for use outside of CI.
#
# This script collects all of the object files provided as arguments and invokes
# the standard CHERIoT `xmake` build workflow, inserting the object paths to our
# compartments' `objectfiles` table.
#
# We use the `objectfiles` table as this will trigger a rebuild of our compartment
# when it differs between builds.
#
# We write/read these from a file `rust_objects.txt` because I couldn't find a
# saner way to communicate these to `xmake`.

set -e

# The current working directory is wherever we called cargo (anywhere). Use the
# absolute path to our build directory, derived from the path to this script.
cd "$(realpath "$(dirname "$0")/../build")"

objects_list="rust_objects.txt"
output_fw="build/cheriot/cheriot/release/test_fw"

rm -f "$objects_list"
rm -f "$output_fw" # just in case

prev_arg=""
output_path=""

for arg in "$@"; do
    # e.g. "-o <FILENAME>"
    case "$prev_arg" in
    -o)
        if [[ "$arg" != /* ]]; then
            echo "Relative paths not handled"
            echo "$arg"
            exit 1
        fi
        output_path="$arg"
        ;;
    esac

    # collect object files
    case "$arg" in
    *.o | *.rlib)
        if [[ "$arg" != /* ]]; then
            echo "Relative paths not handled"
            echo "$arg"
            exit 1
        fi
        echo "$arg" >> "$objects_list"
        ;;
    esac

    prev_arg="$arg"
done

# if we need to clone the rtos
if [[ ! -d "cheriot-rtos" ]]; then
    git clone https://github.com/CHERIoT-Platform/cheriot-rtos --recursive --depth=1
fi

# if we need to run xmake config
if [[ ! -d ".xmake" || ! -d "build" ]]; then
    CHERIOT_LLVM_PROJECT_PATH=${CHERIOT_SYSROOT_DIR:-"../../../build/host/llvm/"}
    xmake config --sdk="$CHERIOT_LLVM_PROJECT_PATH"
fi

xmake build -r

mv "$output_fw" "$output_path"
