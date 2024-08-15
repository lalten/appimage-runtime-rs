#!/bin/bash

set -euxo pipefail

coproc mount_proc { exec "$TEST_APPIMAGE" --appimage-mount; }
# shellcheck disable=SC2154
pid="$mount_proc_PID"
read -r mount <&"${mount_proc[0]}"

# Check that the mount is OK
test -n "$(mount --types fuse.squashfuse)"
test "$(cat "$mount"/other/path/file.txt)" == "I am data"

# The application is waiting to be SIGINTed
kill -SIGINT "$pid"
wait "$pid" || true

# Wait >= the timeout value passed to squashfuse
tail --pid="$(pgrep -f squashfuse)" -f /dev/null

# Check that the mount is gone
test -z "$( ls -A "$mount" )"
test -z "$(mount --types fuse.squashfuse)"
