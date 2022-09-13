# Readme
The purpose of this directory is to host the Gradle plugin that adds tasks for building the native binaries required by `bdk-android`, and building the language bindings files.

The plugin is applied to the `build.gradle.kts` file in `bdk-android` through the `plugins` block:
```kotlin
// bdk-android
plugins {
    id("org.bitcoindevkit.plugins.generate-android-bindings")
}
```

It adds a series of tasks which are brought together into an aggregate task called `buildAndroidLib`. 

This aggregate task:
1. Builds the native libraries using `bdk-ffi`
2. Places them in the correct resource directories
3. Builds the bindings file
