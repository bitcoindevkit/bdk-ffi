#!/usr/bin/env bash
#set -euo pipefail

#pushd bdk-ffi
echo "Confirm bdk-ffi rust library builds"
cargo build --manifest-path ./bdk-ffi/Cargo.toml --release

echo "Generate bdk-ffi Python bindings"
# clean solution once uniffi-bindgen 0.15.3 is released
# uniffi-bindgen generate src/bdk.udl --no-format --out-dir ../src/bdkpython/ --language python

# in the meantime, set UNIFFI_BINDGEN environment variable to a local, latest version of uniffi-rs/uniffi_bindgen/Cargo.toml
# and the BDK_PYTHON environment variable to the current directory
#cd $UNIFFI_BINDGEN/
#cargo run -- generate $BDK_PYTHON/src/bdk.udl --no-format --out-dir ./src/bdkpython/ --language python
#cd -

cargo run --manifest-path $UNIFFI_BINDGEN -- generate ./bdk-ffi/src/bdk.udl --no-format --out-dir ./src/bdkpython/ --language python
cp ./bdk-ffi/target/release/libbdkffi.dylib ./src/bdkpython/
