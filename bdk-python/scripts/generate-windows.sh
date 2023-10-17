#!/usr/bin/env bash

set -euo pipefail
python3 --version
pip install --user -r requirements.txt

echo "Generating bdk.py..."
cd ../bdk-ffi/
cargo run --bin uniffi-bindgen generate src/bdk.udl --language python --out-dir ../bdk-python/src/bdkpython/ --no-format

echo "Generating native binaries..."
rustup default 1.67.0
rustup target add x86_64-pc-windows-msvc
cargo build --profile release-smaller --target x86_64-pc-windows-msvc

echo "Copying libraries bdkffi.dll..."
cp ./target/x86_64-pc-windows-msvc/release-smaller/bdkffi.dll ../bdk-python/src/bdkpython/bdkffi.dll

echo "All done!"
