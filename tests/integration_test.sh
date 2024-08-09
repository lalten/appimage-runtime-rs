#!/bin/bash

set -euxo pipefail

cat <<"EOF" >"$TEST_TMPDIR/AppRun"
#!/bin/sh
echo "Hello World!"
env
EOF
chmod +x "$TEST_TMPDIR/AppRun"

"$MKSQUASHFS" "$TEST_TMPDIR/AppRun" "$TEST_TMPDIR/test.sqfs"

appimage="$TEST_TMPDIR/test.appimage"
cat runtime "$TEST_TMPDIR/test.sqfs" >"$appimage"
chmod +x "$appimage"

out="$TEST_TMPDIR/out"
"$appimage" >"$out"

grep -q "Hello World!" "$out"
grep -q "APPDIR=$TMPDIR/.mount_" "$out"
grep -q "APPIMAGE=$(readlink -f "$appimage")" "$out"
grep -q "ARGV0=$appimage" "$out"
grep -q "OWD=$(pwd)" "$out"
