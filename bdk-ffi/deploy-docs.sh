set -euo pipefail

just docs
cd ./target/doc/
git init .
git switch --create gh-pages
git add .
git commit --message "Deploy $(date +"%Y-%m-%d")"
git remote add upstream git@github.com:bitcoindevkit/bdk-ffi.git
git push upstream gh-pages --force
