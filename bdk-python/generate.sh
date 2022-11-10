#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$(dirname "$(realpath $0)")
PY_SRC="${SCRIPT_DIR}/src/bdkpython/"

echo "Generating bdk.py..."
# GENERATE_PYTHON_BINDINGS_OUT="$PY_SRC" GENERATE_PYTHON_BINDINGS_FIXUP_LIB_PATH=bdkffi cargo run --manifest-path ./bdk-ffi/Cargo.toml --release --bin generate --features generate-python
# BDKFFI_BINDGEN_PYTHON_FIXUP_PATH=bdkffi cargo run --manifest-path ./bdk-ffi/Cargo.toml --package bdk-ffi-bindgen -- --language python --udl-file ./bdk-ffi/src/bdk.udl --out-dir ./src/bdkpython/
BDKFFI_BINDGEN_OUTPUT_DIR="$PY_SRC" BDKFFI_BINDGEN_PYTHON_FIXUP_PATH=bdkffi cargo run --manifest-path ../bdk-ffi/Cargo.toml --package bdk-ffi-bindgen -- --language python --udl-file ../bdk-ffi/src/bdk.udl
