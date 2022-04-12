# Readme
The purpose of this directory is to host the Gradle plugins that add tasks for building the native binaries required by bdk-jvm and bdk-android, and building the language bindings files.

The plugins are applied to the specific `build.gradle.kts` files in `bdk-jvm` and `bdk-android` through the `plugins` block:
```kotlin
// bdk-jvm
plugins {
    id("org.bitcoindevkit.plugin.generate-jvm-bindings")
}

// bdk-android
plugins {
    id("org.bitcoindevkit.plugins.generate-android-bindings")
}
```

They add a series of tasks which are brought together into an aggregate task called `buildJvmLib` for `bdk-jvm` and `buildAndroidLib` for `bdk-android`. 

This aggregate task:
1. Builds the native library(ies) using `bdk-ffi`
2. Places it in the correct resource directory
3. Builds the bindings file
