#!/usr/bin/env bash

set -euo pipefail
python3 --version
pip install -r requirements.txt

cd ../bdk-ffi/
rustup default 1.84.1
rustup target add x86_64-pc-windows-msvc

echo "Generating native binaries..."
cargo build --profile release-smaller --target x86_64-pc-windows-msvc

echo "Generating bdk.py..."
cargo run --bin uniffi-bindgen generate --library ./target/x86_64-pc-windows-msvc/release-smaller/bdkffi.dll --language python --out-dir ../bdk-python/src/bdkpython/ --no-format

echo "Copying libraries bdkffi.dll..."
cp ./target/x86_64-pc-windows-msvc/release-smaller/bdkffi.dll ../bdk-python/src/bdkpython/bdkffi.dll

echo "All done!"
