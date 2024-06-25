# Integration tests for bdk-ffi

This contains simple tests to make sure bdk-ffi can be used as a dependency for each of the 
supported bindings languages.

To skip integration tests and only run unit tests use `cargo test --lib`. 

To run all tests including integration tests use `CLASSPATH=./tests/jna/jna-5.14.0.jar cargo test`. 

Before running integration tests you must install the following development tools:

1. [Java](https://openjdk.org/) and [Kotlin](https://kotlinlang.org/), 
[sdkman](https://sdkman.io/) can help:
```shell
sdk install java 11.0.16.1-zulu
sdk install kotlin 1.7.20`
```
2. [Swift](https://www.swift.org/)
3. [Python](https://www.python.org/)
