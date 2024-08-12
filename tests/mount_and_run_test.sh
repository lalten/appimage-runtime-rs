#!/bin/bash

set -euxo pipefail

out="$TEST_TMPDIR/out"
"$TEST_APPIMAGE" Hello World! | tee "$out"

grep -q "Hello World!" "$out"
grep -q "APPDIR=$TMPDIR/.mount_" "$out"
grep -q "APPIMAGE=$(readlink -f "$TEST_APPIMAGE")" "$out"
grep -q "ARGV0=$TEST_APPIMAGE" "$out"
grep -q "OWD=$(pwd)" "$out"

# Check that the mount point is gone after execution is finished
test ! -d "$TMPDIR/.mount_"

# TODO: check that we don't leave any squashfuse processes behind
