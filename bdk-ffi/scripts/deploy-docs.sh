#!/usr/bin/env bash

# Build the docs site and publish it to the gh-pages branch, which GitHub Pages
# serves at https://bitcoindevkit.github.io/bdk-ffi/.

set -euo pipefail

cd ./target/doc/
git init .
git switch --create gh-pages
git add .
git commit --message "Deploy $(date +"%Y-%m-%d")"
git remote add upstream git@github.com:bitcoindevkit/bdk-ffi.git
git push upstream gh-pages --force
cd ..
