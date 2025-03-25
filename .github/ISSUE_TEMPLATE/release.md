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

3. - [ ] Delete the `target` directory in bdk-ffi and all `build` directories (in root and `lib`) in the bdk-android directory to make sure you're building the library from scratch.
4. - [ ] Build the library and run the offline and live tests, and adjust them if necessary (note that you'll need an Android emulator running).
```shell
# start an emulator prior to running the tests
cd ./bdk-android/
just clean
just build-macos
just test
```
5. - [ ] Update the readme if necessary.

### _JVM_

6. - [ ] Delete the `target` directory in bdk-ffi and all `build` directories (in root and `lib`) in bdk-jvm directory to make sure you're building the library from scratch.
7. - [ ] Build the library and run all offline and live tests, and adjust them if necessary.
```shell
cd ./bdk-jvm/
just clean
just build
just test
```
8. - [ ] Update the readme if necessary.

### _Swift_

9. - [ ] Delete the `target` directory in bdk-ffi.
10. - [ ] Run all offline and live tests and adjust them if necessary.
```shell
cd ./bdk-swift/
just clean
just build
just test
```
11. - [ ] Update the readme if necessary.

### _Python_

12. - [ ] Delete the `dist`, `build`, and `bdkpython.egg-info` and rust `target` directories to make sure you are building the library from scratch without any caches.
13. - [ ] Build the library.
```shell
cd ./bdk-python/
just clean
pip3 install --requirement requirements.txt
bash ./scripts/generate-macos-arm64.sh # run the script for your particular platform
python3 setup.py --verbose bdist_wheel
```
14. - [ ] Run all offline and live tests and adjust them if necessary.
```shell
pip3 install ./dist/bdkpython-<yourversion>-py3-none-any.whl --force-reinstall
python -m unittest --verbose
```
15. - [ ] Update the readme and `setup.py` if necessary.

## Part 3: Release Workflow

16. - [ ] Update the Android, JVM, Python, and Swift libraries as per the _specific libraries' workflows_ sections above. Open a single PR on `master` for all of these changes called `Prepare language bindings libraries for 0.X release`. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/315).
17. - [ ] Create a new branch off of `master` called `release/<feature version>`, e.g. `release/1.2`
18. - [ ] Update bdk-android version from `SNAPSHOT` version to release version
19. - [ ] Update bdk-jvm version from `SNAPSHOT` version to release version
20. - [ ] Update bdk-python version from `.dev` version to release version
21. - [ ] Open a PR to that release branch that updates the Android, JVM, and Python libraries' versions in the three steps above. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/316).
22. - [ ] Get a review and ACK and merge the PR updating all the languages to their release versions on the release branch.
23. - [ ] Create the tag for the release and make sure to add the changelog info to the tag (works better if you prepare the tag message on the side in a text editor). Push the tag to GitHub.
```shell
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
24. - [ ] Trigger manual releases for all 4 libraries (for Swift, go on the [bdk-swift](https://github.com/bitcoindevkit/bdk-swift) repository, and trigger the using `master`. Simply add the version number and tag name in the text fields when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`, but the tag will have it, i.e. `v0.26.0`). Note also that for all 3 other libraries on the bdk-ffi repo, you should trigger the release workflow using the tag (not a branch).
25. - [ ] Make sure the released libraries work and contain the artifacts you would expect.
26. - [ ] Aggregate all the changelog notices from the PRs and add them to the changelog file. PR that.
27. - [ ] Bump the versions on master from `1.1.0-SNAPSHOT` to `1.2.0-SNAPSHOT` (Android + JVM), `1.1.0.dev0` to `1.2.0.dev0` (Python), and `1.1.0-dev` to `1.2.0-alpha.0` (Rust).
28. - [ ] Apply changes to the release issue template if needed.
29. - [ ] Make release on GitHub (generate auto release notes between the previous tag and the new one).
30. - [ ] Build API docs for Android and JVM locally and PR the websites to the bitcoindevkit.org repo.
31. - [ ] Post in the announcement channel.
32. - [ ] Tweet about the new release!
