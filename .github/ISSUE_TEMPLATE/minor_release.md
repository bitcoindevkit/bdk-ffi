---
name: Minor Release
about: Create a new minor release [for release managers only]
title: 'Release MAJOR.MINOR+1.0'
labels: 'release'
assignees: ''
---

## Create a new minor release
### _Main Workflow_
1. - [ ] Open a PR with an update to `Cargo.toml` to the new bdk release candidate and ensure all CI workflows run correctly. Fix errors if necessary.
2. - [ ] Once the new bdk release is out, update the PR to replace the release candidate with the full release and merge.
3. - [ ] Update the Android, JVM, Python, and Swift libraries as per the ["**_Sub-Workflows_**" section below](#Sub-Workflows). Open a single PR on master for all of these changes called `Prepare language bindings libraries for 0.X release`
18. - [ ] Create a new branch off of `master` called `release/version`
19. - [ ] Checkout that branch and open a PR to update the Android, JVM, and Python libraries' versions
  - [ ] Update bdk-android version from `SNAPSHOT` version to release version
  - [ ] Update bdk-jvm version from `SNAPSHOT` version to release version
  - [ ] Update bdk-python version from `.dev` version to release version
20. - [ ] Merge the PR updating all of the languages to their release versions
21. - [ ] Create the tag and make sure to add the changelog info to the tag (works better if you prepare the tag message on the side in a text editor) and push it to GitHub.
```sh
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
22. - [ ] Make release on GitHub (set as pre-release and generate auto release notes between the previous tag and the new one)
23. - [ ] Trigger manual releases for all 4 libraries (for Swift, simply add the version number in the text field when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`)
24. - [ ] Bump the versions on master from `0.9.0-SNAPSHOT` to `0.10.0-SNAPSHOT`, `0.6.0.dev0` to `0.7.0.dev0`
25. - [ ] Build and publish API docs for JVM, Android, and Java on the website
```bash!
./gradlew dokkaHtml    # bdk-jvm (Dokka)
./gradlew dokkaJavadoc # bdk-jvm (java-style documentation)
./gradlew dokkaHtml    # bdk-android (Dokka)
```
26. - [ ] Tweet about the library
27. - [ ] Post in the announcement channel

### _Sub Workflows_
#### _Android_
4. - [ ] Update the API docs to reflect the changes in the API
5. - [ ] Delete the `target` directory in bdk-ffi and all previous artifacts to make sure you're building the library from scratch
6. - [ ] Build the library and run the tests, and adjust if necessary.
```sh
# start an emulator prior to running the tests
cd bdk-android
./gradlew buildAndroidLib
./gradlew connectedAndroidTest
```
7. - [ ] Update the readme if necessary

#### _JVM_
8. - [ ] Update the API docs to reflect the changes in the API
9. - [ ] Delete the `target` directory in bdk-ffi and all previous artifacts to make sure you're building the library from scratch
10. - [ ] Build the library and run the tests, and adjust if necessary
```sh
cd bdk-jvm
./gradlew buildJvmLib
./gradlew test
```
11. - [ ] Update the readme if necessary

#### _Swift_
12. - [ ] Run the tests and adjust if necessary
```sh
./bdk-swift/build-local-swift.sh
cd bdk-swift
swift test
```
13. - [ ] Update the readme if necessary

#### _Python_
14. - [ ] Delete the `.tox`, `dist`, `build`, and `bdkpython.egg-info` and rust `target` directories to make sure you are building the library from scratch without any caches
15. - [ ] Build the library
```shell
pip3 install --requirement requirements.txt
bash ./generate.sh
python3 setup.py --verbose bdist_wheel
```
16. - [ ] Run the tests and adjust if necessary
```shell
pip3 install ./dist/bdkpython-<yourversion>-py3-none-any.whl
python -m unittest --verbose tests/test_bdk.py
```
17. - [ ] Update the readme and `setup.py` if necessary
