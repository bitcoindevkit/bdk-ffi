#!/usr/bin/env bash

set -euo pipefail
${PYBIN}/python --version
${PYBIN}/pip install -r requirements.txt

echo "Generating bdk.py..."
cd ../bdk-ffi/
cargo run --bin uniffi-bindgen generate src/bdk.udl --language python --out-dir ../bdk-python/src/bdkpython/ --no-format

echo "Generating native binaries..."
rustup default 1.77.1
cargo build --profile release-smaller

echo "Copying linux libbdkffi.so..."
cp ./target/release-smaller/libbdkffi.so ../bdk-python/src/bdkpython/libbdkffi.so

echo "All done!"
