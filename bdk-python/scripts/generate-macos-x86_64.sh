#!/usr/bin/env bash

set -euo pipefail
python3 --version
pip install -r requirements.txt

echo "Generating bdk.py..."
cd ../bdk-ffi/
rustup default 1.77.1
cargo run --bin uniffi-bindgen generate src/bdk.udl --language python --out-dir ../bdk-python/src/bdkpython/ --no-format

echo "Generating native binaries..."
rustup target add x86_64-apple-darwin
cargo build --profile release-smaller --target x86_64-apple-darwin

echo "Copying libraries libbdkffi.dylib..."
cp ./target/x86_64-apple-darwin/release-smaller/libbdkffi.dylib ../bdk-python/src/bdkpython/libbdkffi.dylib

echo "All done!"
