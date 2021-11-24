# bdk-swift

This project builds a swift package that provides swift language bindings to the [`bdk`] 
library. The swift language bindings are created by the [`bdk-ffi`] project which is 
included as a git submodule of this repository. 

## How to Use

To use the swift language bindings provided by the [`bdk-ffi`] project in your xcode iOS 
or MacOS project add this github repository and select one of the published versions. You 
may then import the `BitcoinDevKit` library. For example:

```swift
import BitcoinDevKit

...

let desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
let config = DatabaseConfig.memory(junk: "")
let wallet = try OfflineWallet.init(descriptor: desc, network: Network.regtest, databaseConfig: config)
let address = wallet.getNewAddress()
```

## How to Build and Publish

If you are a maintainer of this project or want to build and publish this project to your 
own repo use the following steps:

1. Clone this repo and init and update it's [`bdk-ffi`] submodule
   ```shell
   git clone https://github.com/bitcoindevkit/bdk-swift
   git submodule update --init
   ```

1. Follow the "General" `bdk-ffi` ["Getting Started (Developer)"] instructions.

1. Install the latest version of xcode, download and install the advanced tools.

1. Ensure Swift is installed

1. Install required targets
   ```shell
    rustup target add aarch64-apple-ios x86_64-apple-ios
    ```
    
1. Build [`bdk-ffi`] swift bindings and bdkFFI.xcframework.zip
   ```shell
   ./build.sh
   ```

1. Update the `Package.swift` file with the new expected URL for the 
   `bdkFFI.xcframework.zip` file and new hash as shown at the end of the build.sh script.
   For example: 
   ```swift
           .binaryTarget(
            name: "bdkFFI",
            url: "https://github.com/bitcoindevkit/bdk-swift/releases/download/0.1.3/bdkFFI.xcframework.zip",
            checksum: "c0b1e3ea09376b3f316d7d83575e1cd513fc4ad39ef8cf01120a3a1d7757fb97"),
   ```
1. Commit the changed `Package.swift` and tag it with the new version number.
   ```shell
   git add Package.swift
   git commit -m "Bump version to 0.1.3"
   git tag 0.1.3 -m "Release 0.1.3"
   git push --tags
   ```

1. Create a github release for your new tag.

1. Upload the newly created zip to the new github release.

1. Tests the new package in xcode.

[`bdk`]: https://github.com/bitcoindevkit/bdk
[`bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
["Getting Started (Developer)"]: https://github.com/bitcoindevkit/bdk-ffi#getting-started-developer
