#!/bin/bash

set -euxo pipefail

coproc mount_proc { exec "$TEST_APPIMAGE" --appimage-mount; }
# shellcheck disable=SC2154
pid="$mount_proc_PID"
read -r mount <&"${mount_proc[0]}"

# Check that the mount point is OK
test "$(cat "$mount"/other/path/file.txt)" == "I am data"

kill -SIGINT "$pid"
wait "$pid" || true

# Check that the mount point is gone
#TODO: fix
# test ! -d "$mount"

# TODO: check that we don't leave any squashfuse processes behind
