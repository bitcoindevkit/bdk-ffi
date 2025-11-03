---
name: Release
about: Create a new release [for release managers only]
title: "Release MAJOR.MINOR.PATCH"
labels: "release"
assignees: ""
---

## Part 1: Bump BDK Rust Version

1. - [ ] Open a PR with an update to `Cargo.toml` to the new bdk release candidate and ensure all CI workflows run correctly. Fix errors if necessary.
2. - [ ] Once the new bdk release is out, update the PR to replace the release candidate with the full release and merge.

## Part 2: Prepare Libraries for Release Branch

### _Android_

3. - [ ] Delete the `target` directory in bdk-ffi and all `build` directories (in root and `lib`) in the bdk-android directory to make sure you're building the library from scratch.
4. - [ ] Build the library and run the offline and live tests, and adjust them if necessary (note that you'll need an Android emulator running).
```shell
# start an emulator prior to running the tests
cd ./bdk-android/
just clean
just build
just test
```
5. - [ ] Update the readme if necessary.

### _Swift_

6. - [ ] Delete the `target` directory in bdk-ffi.
7. - [ ] Run all offline and live tests and adjust them if necessary.
```shell
cd ./bdk-swift/
just clean
just build
just test
```
8. - [ ] Update the readme if necessary.

## Part 3: Release Workflow

9. - [ ] Update the Android and Swift libraries as per the _Part 2_ section above. Open a single PR on `master` for all of these changes called `Prepare language bindings libraries for 0.X release`. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/315).
10. - [ ] Create a new branch off of `master` called `release/<feature version>`, e.g. `release/1.2`
11. - [ ] Update bdk-android version from `SNAPSHOT` version to release version and open a PR to the release branch that updates the Android version. See [example PR here](https://github.com/bitcoindevkit/bdk-ffi/pull/316).
12. - [ ] Get a review and ACK and merge this PR on the release branch.
13. - [ ] Create the tag for the release and make sure to add the changelog info to the tag (works better if you prepare the tag message on the side in a text editor). Push the tag to GitHub.
```shell
git tag v0.6.0 --sign --edit
git push upstream v0.6.0
```
14. - [ ] Trigger manual releases for both libraries (for Swift, go to the [bdk-swift](https://github.com/bitcoindevkit/bdk-swift) repository and trigger the workflow using `master`. Simply add the version number and tag name in the text fields when running the workflow manually. Note that the version number must not contain the `v`, i.e. `0.26.0`, but the tag will have it, i.e. `v0.26.0`). For Android, trigger the release workflow using the tag (not a branch).
15. - [ ] Make sure the released libraries work and contain the artifacts you would expect.
16. - [ ] Aggregate all the changelog notices from the PRs and add them to the changelog file. PR that.
17. - [ ] Bump the version on master from `1.1.0-SNAPSHOT` to `1.2.0-SNAPSHOT` (Android) and `1.1.0-alpha.0` to `1.2.0-alpha.0` (Rust).
18. - [ ] Apply changes to the release issue template if needed.
19. - [ ] Make release on GitHub (generate auto release notes between the previous tag and the new one).
20. - [ ] Build API docs for Android locally and PR the website to the bitcoindevkit.org repo.
21. - [ ] Post in the announcement channel.
22. - [ ] Tweet about the new release!
