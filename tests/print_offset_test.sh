#!/bin/bash

set -euxo pipefail

runtime_size="$(stat --printf="%s" "$(readlink -f "$RUNTIME_PATH")")"

test "$("$TEST_APPIMAGE" --appimage-offset)" == "$runtime_size"
