---
name: Minor Release
about: Create a new minor release [for release managers only]
title: 'Release MAJOR.MINOR+1.0'
labels: 'release'
assignees: ''
---

## Create a new minor release
## Bumping BDK Rust Version
1. - [ ] Open a PR with an update to `Cargo.toml` to the new bdk release candidate and ensure all CI workflows run correctly. Fix errors if necessary.
2. - [ ] Once the new bdk release is out, update the PR to replace the release candidate with the full release and merge.

### Specific Libraries' Workflows
#### _Android_
3. - [ ] Update the API docs to reflect the changes in the API
4. - [ ] Delete the `target` directory in bdk-ffi and all previous artifacts to make sure you're building the library from scratch.
5. - [ ] Build the library and run the tests, and adjust if necessary.
```sh
# start an emulator prior to running the tests
cd ./bdk-android/
./gradlew buildAndroidLib
./gradlew connectedAndroidTest
```
6. - [ ] Update the readme if necessary
#### _JVM_
7. - [ ] Update the API docs to reflect the changes in the API
8. - [ ] Delete the `target` directory in bdk-ffi and all previous artifacts to make sure you're building the library from scratch
9. - [ ] Build the library and run the tests, and adjust if necessary
```sh
cd ./bdk-jvm/
./gradlew buildJvmLib
./gradlew test
```
10. - [ ] Update the readme if necessary
#### _Swift_
11. - [ ] Run the tests and adjust if necessary
```sh
./bdk-swift/build-local-swift.sh
cd ./bdk-swift/
swift test
```
12. - [ ] Update the readme if necessary
#### _Python_
13. - [ ] Delete the `.tox`, `dist`, `build`, and `bdkpython.egg-info` and rust `target` directories to make sure you are building the library from scratch without any caches
14. - [ ] Build the library
```shell
cd ./bdk-python/
pip3 install --requirement requirements.txt
bash ./generate.sh
python3 setup.py --verbose bdist_wheel
```
15. - [ ] Run the tests and adjust if necessary
```shell
pip3 install ./dist/bdkpython-<yourversion>-py3-none-any.whl --force-reinstall
python -m unittest --verbose tests/test_bdk.py
```
16. - [ ] Update the readme and `setup.py` if necessary

### Release Workflow
17. - [ ] Update the Android, JVM, Python, and Swift libraries as per the _Specific Libraries' Workflows_ section above. Open a single PR on master for all of these changes called `Prepare language bindings libraries for 0.X release`. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/315).
- [ ] Create a new branch off of `master` called `release/version`
18. - [ ] Open a PR to that branch to update the Android, JVM, and Python libraries' versions. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/316).
    - [ ] Update bdk-android version from `SNAPSHOT` version to release version
    - [ ] Update bdk-jvm version from `SNAPSHOT` version to release version
    - [ ] Update bdk-python version from `.dev` version to release version
19. - [ ] Merge the PR updating all the languages to their release versions
20. - [ ] Create the tag and make sure to add the changelog info to the tag (works better if you prepare the tag message on the side in a text editor) and push it to GitHub.
```sh
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
21. - [ ] Aggregate all the changelog notices from the PRs and add them to the changelog file
22. - [ ] Open a PR on master with the changes to the changelog file and the development versions bump. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/317).
23. - [ ] Make release on GitHub (set as pre-release and generate auto release notes between the previous tag and the new one)
24. - [ ] Trigger manual releases for all 4 libraries (for Swift, simply add the version number in the text field when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`)
25. - [ ] Bump the versions on master from `0.9.0-SNAPSHOT` to `0.10.0-SNAPSHOT`, `0.6.0.dev0` to `0.7.0.dev0`
26. - [ ] Build and publish API docs for JVM, Android, and Java on the website
```bash!
./gradlew dokkaHtml    # bdk-jvm (Dokka)
./gradlew dokkaJavadoc # bdk-jvm (java-style documentation)
./gradlew dokkaHtml    # bdk-android (Dokka)
```
27. - [ ] Tweet about the library
28. - [ ] Post in the announcement channel
