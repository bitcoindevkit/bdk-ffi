#!/usr/bin/env bash

set -euo pipefail

# Rustdoc only emits module pages for publicly reachable modules. For the docs to
# build correctly, we temporarily rewrite the top-level `mod foo;` lines into `pub mod foo;`.
sed -i '' -E 's/^mod (bitcoin|descriptor|electrum|error|esplora|keys|kyoto|macros|store|tx_builder|types|wallet);$/pub mod \1;/' src/lib.rs

# We always restore the original `src/lib.rs` on exit so the temporary visibility
# change never sticks around after the docs build finishes.
cleanup() {
  sed -i '' -E 's/^pub mod (bitcoin|descriptor|electrum|error|esplora|keys|kyoto|macros|store|tx_builder|types|wallet);$/mod \1;/' src/lib.rs
}
trap cleanup EXIT

# Build the docs
rm -rf target/doc
cargo doc --no-deps
printf '%s\n' '<meta http-equiv="refresh" content="0; url=bdkffi/index.html">' > target/doc/index.html
printf 'Documentation built at target/doc/\n'
