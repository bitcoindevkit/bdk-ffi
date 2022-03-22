# Readme
The purpose of this directory is to host a Gradle plugin that adds tasks for building the native binaries required by bdk-jvm/ bdk-android and building the language bindings files.

The plugin is applied to the specific `build.gradle.kts` files in `bdk-jvm` and `bdk-android` through the `plugins` block:
```kotlin
plugins {
    id("org.bitcoindevkit.plugin.generate-bdk-bindings")
}
```

It adds a series of tasks (`buildJvmBinary`, `moveNativeJvmLib`, `generateJvmBindings`) which are then brought together into an aggregate task called `buildJvmLib`.

This task:
1. Builds the native JVM library (on your given platform) using `bdk-ffi`
2. Places it in the correct resource directory
3. Builds the bindings file
