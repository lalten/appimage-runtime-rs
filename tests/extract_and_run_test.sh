#!/bin/bash

set -euxo pipefail

out="$TEST_TMPDIR/out"
"$TEST_APPIMAGE" --appimage-extract-and-run Hello World! | tee "$out"

grep -q "Hello World!" "$out"
grep -q -E "APPDIR=$TMPDIR/appimage_extracted_[0-9a-zA-Z]+" "$out"
grep -q "APPIMAGE=$(readlink -f "$TEST_APPIMAGE")" "$out"
grep -q "ARGV0=$TEST_APPIMAGE" "$out"
grep -q "OWD=$(pwd)" "$out"
