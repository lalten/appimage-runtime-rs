#!/bin/bash

set -euxo pipefail

# AppImage magic bytes: 0x41 0x49 0x02
grep -q --binary -P "^AI\x02$" <(dd if="$RUNTIME" skip=8 bs=1 count=3 2>/dev/null)

# Adding the magic should not change the size of the runtime binary
runtime_binary_size="$(stat --printf="%s" "$(readlink -f "$RUNTIME_BIN")")"
runtime_wmagic_size="$(stat --printf="%s" "$(readlink -f "$RUNTIME")")"
test "$runtime_binary_size" == "$runtime_wmagic_size"
