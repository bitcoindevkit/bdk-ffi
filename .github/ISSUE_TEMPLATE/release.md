---
name: Release
about: Create a new release [for release managers only]
title: "Release MAJOR.MINOR.PATCH"
labels: "release"
assignees: ""
---

## Part 1: Bump BDK Rust Version

- [ ] Open a PR with an update to `Cargo.toml` to the new bdk release candidate and ensure all CI workflows run correctly. Fix errors if necessary.
- [ ] Once the new bdk release is out, update the PR to replace the release candidate with the full release and merge.

## Part 2: Prepare Libraries for Release Branch

### _Android_

- [ ] Delete the `target` directory in bdk-ffi and all `build` directories (in root and `lib`) in the bdk-android directory to make sure you're building the library from scratch.
- [ ] Build the library and run the offline and live tests, and adjust them if necessary (note that you'll need an Android emulator running).
```shell
# start an emulator prior to running the tests
cd ./bdk-android/
just clean
just build
just test
```
- [ ] Update the readme if necessary.

### _Swift_

- [ ] Delete the `target` directory in bdk-ffi.
- [ ] Run all offline and live tests and adjust them if necessary.
```shell
cd ./bdk-swift/
just clean
just build
just test
```
- [ ] Update the readme if necessary.

## Part 3: Release Workflow

- [ ] Update the Android and Swift libraries as per the _Part 2_ section above. Open a single PR on `master` for all of these changes called `Prepare language bindings libraries for 0.X release`. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/315).
- [ ] Create a new branch off of `master` called `release/<feature version>`, e.g. `release/1.2`
- [ ] Generate release notes with GitHub's built-in release note generator, using the previous release tag as the previous tag and the release branch as the target. Review the generated notes and prepare the final changelog text on the side in a text editor.
- [ ] Update the bdk-android version from a `SNAPSHOT` version to a release version (`2.4.0-SNAPSHOT` -> `2.4.0`), update the Rust library version in `Cargo.toml` from an alpha to a release version (`2.4.0-alpha.0` -> `2.4.0`), add the reviewed release notes to the changelog file, and open a PR to the release branch with these changes.
- [ ] Get a review and ACK and merge this PR on the release branch.
- [ ] Create the tag for the release and make sure to add the reviewed changelog info to the tag. Push the tag to GitHub.
```shell
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
- [ ] Update all downstream libraries (dart, rn, python, and jvm) to the given tag. This allows for everyone to test their own workflows, run their CI, etc.
- [ ] Trigger releases for both libraries (for Swift, go to the [bdk-swift](https://github.com/bitcoindevkit/bdk-swift) repository and trigger the workflow using `master`. Simply add the version number and tag name in the text fields when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`, but the tag will have it, i.e. `v0.26.0`). For Android, trigger the release locally.
- [ ] Make sure the released libraries work and contain the artifacts you would expect.
- [ ] Build the Rust API docs and publish them to the repo's GitHub Pages.
```shell
cd bdk-ffi/
just docs
```
- [ ] Bump the version on master from `1.1.0-SNAPSHOT` to `1.2.0-SNAPSHOT` (Android) and `1.1.0-alpha.0` to `1.2.0-alpha.0` (Rust).
- [ ] Apply changes to the release issue template if needed.
- [ ] Make release on GitHub using the reviewed release notes.
- [ ] Post in the announcement channel.
- [ ] Tweet about the new release!
