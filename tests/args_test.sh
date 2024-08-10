#!/bin/bash

set -euxo pipefail

out="$TEST_TMPDIR/out"

"$TEST_APPIMAGE" --appimage-help | tee "$out"
grep -q "Usage: $TEST_APPIMAGE" "$out"


if 2>&1 "$TEST_APPIMAGE" --appimage-INVALID_OPTION | tee "$out"; then
    echo "Expected failure but got success"
    exit 1
else
    grep -q "Invalid --appimage- arg" "$out"
fi
