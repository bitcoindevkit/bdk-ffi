---
name: Release
about: Create a new release [for release managers only]
title: 'Release MAJOR.MINOR.PATCH'
labels: 'release'
assignees: ''
---

# Part 1: Bump BDK Rust Version

1. - [ ] Open a PR with an update to `Cargo.toml` to the new bdk release candidate and ensure all CI workflows run correctly. Fix errors if necessary.
2. - [ ] Once the new bdk release is out, update the PR to replace the release candidate with the full release and merge.

# Part 2: Prepare Libraries for Release Branch

### _Android_

3. - [ ] Update the API docs to reflect the changes in the API
4. - [ ] Delete the `target` directory in bdk-ffi and all `build` directories (in root, `lib`, and `plugins`) in the bdk-android directory to make sure you're building the library from scratch.
5. - [ ] Build the library and run the offline and live tests, and adjust them if necessary (note that you'll need an Android emulator running).
```shell
# start an emulator prior to running the tests
cd ./bdk-android/
just clean
just build
just test
```
6. - [ ] Update the readme if necessary

### _JVM_

7. - [ ] Update the API docs to reflect the changes in the API
8. - [ ] Delete the `target` directory in bdk-ffi and all `build` directories (in root, `lib`, and `plugins`) in bdk-jvm directory to make sure you're building the library from scratch.
9. - [ ] Build the library and run the tests, and adjust if necessary
```shell
cd ./bdk-jvm/
just clean
just build
just test
```
10.  - [ ] Update the readme if necessary

### _Swift_

11. - [ ] Delete the `target` directory in bdk-ffi
12. - [ ] Run the tests and adjust if necessary
```shell
cd ./bdk-swift/
just clean
just build
just test
```
13.  - [ ] Update the readme if necessary

### _Python_

14. - [ ] Delete the `dist`, `build`, and `bdkpython.egg-info` and rust `target` directories to make sure you are building the library from scratch without any caches
15. - [ ] Build the library
```shell
cd ./bdk-python/
just clean
pip3 install --requirement requirements.txt
bash ./scripts/generate-macos-arm64.sh # run the script for your particular platform
python3 setup.py --verbose bdist_wheel
```
16.  - [ ] Run the tests and adjust if necessary
```shell
pip3 install ./dist/bdkpython-<yourversion>-py3-none-any.whl --force-reinstall
python -m unittest --verbose
```
17.  - [ ] Update the readme and `setup.py` if necessary
18. - [ ] Update the Android, JVM, Python, and Swift libraries as per the _Specific Libraries' Workflows_ section above. Open a single PR on master for all of these changes called `Prepare language bindings libraries for 0.X release`. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/315).

## Part 3: Release Workflow

19. - [ ] Create a new branch off of `master` called `release/<feature version>`, e.g. `release/0.31`
20. - [ ] Update bdk-android version from `SNAPSHOT` version to release version
21. - [ ] Update bdk-jvm version from `SNAPSHOT` version to release version
22. - [ ] Update bdk-python version from `.dev` version to release version
23. - [ ] Open a PR to that release branch that updates the Android, JVM, and Python libraries' versions in the three steps above. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/316).
24. - [ ] Get a review and ACK and merge the PR updating all the languages to their release versions
25. - [ ] Create the tag for the release and make sure to add the changelog info to the tag (works better if you prepare the tag message on the side in a text editor). Push the tag to GitHub.
```shell
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
26.  - [ ] Trigger manual releases for all 4 libraries (for Swift, go on the [bdk-swift](https://github.com/bitcoindevkit/bdk-swift) trigger the release on `master` and simply add the version number and tag name in the text fields when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`, but the tag will have it, i.e. `v0.26.0`). Note that on bdk-ffi to trigger release with tag (not branch).
27.  - [ ] Make sure the released libraries work and contain the artifacts you would expect 
28.  - [ ] Aggregate all the changelog notices from the PRs and add them to the changelog file
29.  - [ ] Bump the versions on master from `0.9.0-SNAPSHOT` to `0.10.0-SNAPSHOT`, `0.6.0.dev0` to `0.7.0.dev0`
30.  - [ ] Apply changes to the release issue template if needed
31.  - [ ] Make release on GitHub (set as pre-release and generate auto release notes between the previous tag and the new one)
32.  - [ ] Post in the announcement channel
33.  - [ ] Tweet about the library
