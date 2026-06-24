#!/bin/sh

set -e

echo "something evil"

for d in ./* ; do
    if [ -d "$d" ]; then
        cd $d && ./build_and_run.sh
    fi
done
