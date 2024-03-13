#!/usr/bin/env bash

set -euo pipefail
python3 --version
pip install --user -r requirements.txt

echo "Generating bdk.py..."
cd ../bdk-ffi/
cargo run --bin uniffi-bindgen generate src/bdk.udl --language python --out-dir ../bdk-python/src/bdkpython/ --no-format

echo "Generating native binaries..."
rustup default 1.74.0
rustup target add aarch64-apple-darwin
cargo build --profile release-smaller --target aarch64-apple-darwin

echo "Copying libraries libbdkffi.dylib..."
cp ./target/aarch64-apple-darwin/release-smaller/libbdkffi.dylib ../bdk-python/src/bdkpython/libbdkffi.dylib

echo "All done!"
