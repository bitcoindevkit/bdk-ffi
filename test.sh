#!/usr/bin/env bash
set -eo pipefail -o xtrace

# rust
cargo test --features c-headers -- generate_headers

# cc
export LD_LIBRARY_PATH=`pwd`/target/debug
#valgrind --leak-check=full --show-leak-kinds=all cc/bdk_ffi_test
cc/bdk_ffi_test

## bdk-kotlin
(cd bdk-kotlin && gradle test)
(cd bdk-kotlin && gradle :android:connectedDebugAndroidTest)
