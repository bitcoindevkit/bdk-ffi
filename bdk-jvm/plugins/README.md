# Readme
The purpose of this directory is to host the Gradle plugin that adds tasks for building the native binaries required by bdk-jvm, and building the language bindings files.

The plugin is applied to the `build.gradle.kts` file through the `plugins` block:
```kotlin
plugins {
    id("org.bitcoindevkit.plugin.generate-jvm-bindings")
}
```

The plugin adds a series of tasks which are brought together into an aggregate task called `buildJvmLib` for `bdk-jvm`. 

This aggregate task:
1. Builds the native library(ies) using `bdk-ffi`
2. Places it in the correct resource directory
3. Builds the bindings file
