#!/bin/bash

set -euxo pipefail

cat <<"EOF" >"$TEST_TMPDIR/AppRun"
#!/bin/sh
echo "$@"
env
EOF
chmod +x "$TEST_TMPDIR/AppRun"

"$MKSQUASHFS" "$TEST_TMPDIR/AppRun" "$TEST_TMPDIR/test.sqfs"

appimage="$TEST_TMPDIR/test.appimage"
cat runtime "$TEST_TMPDIR/test.sqfs" >"$appimage"
chmod +x "$appimage"

out="$TEST_TMPDIR/out"
"$appimage" Hello World! | tee "$out"

grep -q "Hello World!" "$out"
grep -q "APPDIR=$TMPDIR/.mount_" "$out"
grep -q "APPIMAGE=$(readlink -f "$appimage")" "$out"
grep -q "ARGV0=$appimage" "$out"
grep -q "OWD=$(pwd)" "$out"

"$appimage" --appimage-help | tee "$out"
grep -q "Usage: $appimage" "$out"

runtime_size="$(stat --printf="%s" "$(readlink -f runtime)")"
test "$("$appimage" --appimage-offset)" == "$runtime_size"

if 2>&1 "$appimage" --appimage-INVALID_OPTION | tee "$out"; then
    echo "Expected failure but got success"
    exit 1
else
    grep -q "Invalid --appimage- arg" "$out"
fi
