[group("Repo")]
[doc("Default command; list all available commands.")]
@list:
  just --list --unsorted

[group("Repo")]
[doc("Open repo on GitHub in your default browser.")]
repo:
  open https://github.com/bitcoindevkit/bdk-ffi

[group("Build")]
[doc("Remove all caches and previous builds to start from scratch.")]
clean:
  rm -rf ../bdk-ffi/target/
  rm -rf ./bdkpython.egg-info/
  rm -rf ./build/
  rm -rf ./dist/

[group("Test")]
[doc("Run all tests.")]
test:
  python3 -m unittest --verbose
