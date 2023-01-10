# bdk-swift

This project builds a Swift package that provides [Swift] language bindings for the
[`bdk`] library. The Swift language bindings are created by the [`bdk-ffi`] project which is included as a module of this repository.

Supported target platforms are:

- macOS, X86_64 and M1 (aarch64)  
- iOS, iPhones (aarch64)  
- iOS simulator, X86_64 and M1 (aarch64)  

## How to Use

To use the Swift language bindings for [`bdk`] in your [Xcode] iOS or macOS project add
the GitHub repository https://github.com/bitcoindevkit/bdk-swift and select one of the
release versions. You may then import and use the `BitcoinDevKit` library in your Swift
code. For example:

```swift
import BitcoinDevKit

...

```

Swift Package Manager releases for `bdk-swift` are published to a separate repository (https://github.com/bitcoindevkit/bdk-swift), and that is where the releases are created for it. 

The `bdk-swift/build-local-swift.sh` script can be used instead to create a version of the project for local testing.

### How to test

```shell
swift test
```

### Example Projects

* [BdkSwiftSample](https://github.com/futurepaul/BdkSwiftSample), iOS

## How to Build and Publish

If you are a maintainer of this project or want to build and publish this project to your
own GitHub repository use the following steps:

1. If it doesn't already exist, create a new `release/0.MINOR` branch from the `master` branch.
2. Add a tag `v0.MINOR.PATCH`.
3. Run the `publish-spm` workflow on GitHub from the `bdk-swift` repo for  version `0.MINOR.PATCH`.

[Swift]: https://developer.apple.com/swift/
[Xcode]: https://developer.apple.com/documentation/Xcode
[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
