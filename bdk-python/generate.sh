#!/usr/bin/env bash

set -euo pipefail

echo "Generating bdk.py..."
cd ../bdk-ffi/
cargo run --features uniffi/cli --bin uniffi-bindgen generate src/bdk.udl --language python --out-dir ../bdk-python/src/bdkpython/ --no-format
cargo build --features uniffi/cli --profile release-smaller

echo "Generating native binaries..."
mv ../target/release-smaller/libbdkffi.dylib ../bdk-python/src/bdkpython/libbdkffi.dylib

echo "Bundling bdkpython..."
cd ../bdk-python/
python3 setup.py --verbose bdist_wheel

echo "All done!"
