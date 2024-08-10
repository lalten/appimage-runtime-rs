#!/bin/bash

set -euxo pipefail

out="$TEST_TMPDIR/out"
"$TEST_APPIMAGE" --appimage-list | tee "$out"

expected="$TEST_TMPDIR/expected"
cat <<EOF > "$expected"
/
/AppRun
EOF

diff -u "$expected" "$out"
