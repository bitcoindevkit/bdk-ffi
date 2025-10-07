---
name: Release
about: Create a new release [for release managers only]
title: "Release MAJOR.MINOR.PATCH"
labels: "release"
assignees: ""
---

# Part 1: Bump BDK Rust Version

1. - [ ] Open a PR with an update to `Cargo.toml` to the new bdk release candidate and ensure all CI workflows run correctly. Fix errors if necessary.
2. - [ ] Once the new bdk release is out, update the PR to replace the release candidate with the full release and merge.

# Part 2: Prepare Libraries for Release Branch

### _Android_

3. - [ ] Delete all previous build artifacts in order to build and test the library from scratch.
4. - [ ] Build the library and run the tests (note that you'll need an Android emulator running). Adjust them if necessary.
```shell
# start an emulator prior to running the tests
cd ./bdk-android/
just clean
just build
just test
```
5. - [ ] Update the readme if necessary.

### _Swift_

6. - [ ] Delete all previous build artifacts in order to build and test the library from scratch.
7. - [ ] Build the library and run the tests. Adjust them if necessary.
```shell
cd ./bdk-swift/
just clean
just build
just test
```
8. - [ ] Update the readme if necessary.

## Part 3: Release Workflow

9.  - [ ] Update the Android and Swift libraries as per the _specific libraries' workflows_ sections above. Open a single PR on `master` for all of these changes called `Prepare language bindings libraries for 0.X release`. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/315).
10.  - [ ] Create a new branch off of `master` called `release/<feature version>`, e.g. `release/1.2`
11.  - [ ] Update bdk-android version from `SNAPSHOT` version to release version
12.  - [ ] Open a PR to that release branch that updates the Android library version in the step above. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/316).
13.  - [ ] Get a review and ACK and merge the PR updating Android to its release version on the release branch.
14.  - [ ] Create the tag for the release and make sure to add the changelog info to the tag (works better if you prepare the tag message on the side in a text editor). Push the tag to GitHub.
```shell
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
15.  - [ ] Trigger manual releases for both libraries (for Swift, go on the [bdk-swift](https://github.com/bitcoindevkit/bdk-swift) repository, and trigger the using `master`. Simply add the version number and tag name in the text fields when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`, but the tag will have it, i.e. `v0.26.0`). Note also that for the Android library, you should trigger the release workflow using the tag (not a branch).
16.  - [ ] Make sure the released libraries work and contain the artifacts you would expect.
17.  - [ ] Aggregate all the changelog notices from the PRs and add them to the changelog file. PR that.
18.  - [ ] Bump the version on master from `1.1.0-SNAPSHOT` to `1.2.0-SNAPSHOT` (Android) and `1.1.0-dev` to `1.2.0-alpha.0` (Rust).
19.  - [ ] Apply changes to the release issue template if needed.
20.  - [ ] Make release on GitHub (generate auto release notes between the previous tag and the new one).
21.  - [ ] Trigger the workflow that builds the API docs for Android and PR the website to the bitcoindevkit.org repo.
22.  - [ ] Post in the announcement channel.
23.  - [ ] Tweet about the new releases!
