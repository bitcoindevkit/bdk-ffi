#!/usr/bin/env bash

set -euo pipefail
OS=$(uname -s)

echo "Generating bdk.py..."
cd ../bdk-ffi/
cargo run --features uniffi/cli --bin uniffi-bindgen generate src/bdk.udl --language python --out-dir ../bdk-python/src/bdkpython/ --no-format

echo "Generating native binaries..."
cargo build --features uniffi/cli --profile release-smaller
case $OS in
  "Darwin")
    echo "Copying macOS libbdkffi.dylib..."
    cp ../target/release-smaller/libbdkffi.dylib ../bdk-python/src/bdkpython/libbdkffi.dylib
    ;;
  "Linux")
    echo "Copying linux libbdkffi.so..."
    cp ../target/release-smaller/libbdkffi.so ../bdk-python/src/bdkpython/libbdkffi.so
    ;;
esac

# echo "Bundling bdkpython..."
cd ../bdk-python/
# python setup.py --verbose bdist_wheel

echo "All done!"
