#!/bin/sh

set -e

for d in ./* ; do
    if [ -d "$d" ]; then
        cd $d && ./build_and_run.sh
    fi
done
