#!/bin/bash

set -euxo pipefail

# The squashfs-root dir is created on extraction
test ! -f squashfs-root

# Extract the AppImage contents
"$TEST_APPIMAGE" --appimage-extract

# Files are created and permission bits are preserved
test "0$(stat --format=%a squashfs-root/AppRun)" == 0755
test "0$(stat --format=%a squashfs-root/other/path/file.txt)" == 0644

# The extracted script works just the same (i.e. has the same content)
out="$TEST_TMPDIR/out"
squashfs-root/AppRun Hello World! | tee "$out"
grep -q "args: Hello World!" "$out"

# But the environment variables who are set by the runtime are not present
grep -q -v "APPDIR=" "$out"
grep -q -v "APPIMAGE=" "$out"
grep -q -v "ARGV0=" "$out"
grep -q -v "OWD=" "$out"

# The test squashfs is generated with this SOURCE_DATE_EPOCH set. Extracted files have their mtime set correctly.
SOURCE_DATE_EPOCH=1424879120
test "$(stat --format=%Y squashfs-root/AppRun)" == "$SOURCE_DATE_EPOCH"
test "$(stat --format=%Y squashfs-root/other/path/file.txt)" == "$SOURCE_DATE_EPOCH"

# Extract the AppImage contents with a pattern filter
rm -rf squashfs-root
"$TEST_APPIMAGE" --appimage-extract "*.txt"
test "$(find squashfs-root -type f)" == "squashfs-root/other/path/file.txt"
rm -rf squashfs-root
"$TEST_APPIMAGE" --appimage-extract "**/*Run"
test "$(find squashfs-root -type f)" == "squashfs-root/AppRun"
