# bdk-swift

This project builds a Swift package that provides [Swift] language bindings for the 
[`bdk`] library. The Swift language bindings are created by the [`bdk-ffi`] project which 
is included as a git submodule of this repository. 

## How to Use

To use the Swift language bindings for [`bdk`] in your [Xcode] iOS or MacOS project add 
the github repository (https://github.com/bitcoindevkit/bdk-swift) and select one of the 
release versions. You may then import and use the `BitcoinDevKit` library in your Swift 
code. For example:

```swift
import BitcoinDevKit

...
let desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
let databaseConfig = DatabaseConfig.memory
let wallet = try Wallet.init(descriptor: desc, changeDescriptor: nil, network: Network.regtest, databaseConfig: databaseConfig)
let addressInfo = try wallet.getAddress(addressIndex: AddressIndex.new)
```

### Example Projects

* [BdkSwiftSample](https://github.com/futurepaul/BdkSwiftSample)

## How to Build and Publish

If you are a maintainer of this project or want to build and publish this project to your 
own Github repository use the following steps:

1. If it doesn't already exist, create a new `release/0.MINOR` branch from the `master` branch
2. Run the `publish-spm` workflow on Github for branch `release/0.MINOR` and version `0.MINOR.0`
3. Copy the changelog from corresponding `bdk-ffi` release description to this release

[Swift]: https://developer.apple.com/swift/
[Xcode]: https://developer.apple.com/documentation/Xcode
[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
["Getting Started (Developer)"]: https://github.com/bitcoindevkit/bdk-ffi#getting-started-developer
