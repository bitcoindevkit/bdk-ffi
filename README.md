# BDK UniFFI Language Bindings

## Setup Android build environment

   1. Add Android rust targets

   ```sh
   rustup target add x86_64-apple-darwin x86_64-unknown-linux-gnu x86_64-linux-android aarch64-linux-android armv7-linux-androideabi i686-linux-android
   ```

   2. Set ANDROID_NDK_HOME
   
   ```sh
   export ANDROID_NDK_HOME=/home/<user>/Android/Sdk/ndk/<NDK version, ie. 21.4.7075529>
   ```

## Setup Swift build environment

    1. install Swift, see ["Download Swift"](https://swift.org/download/) page  
       (or on Mac OSX install the latest Xcode)

## Setup UniFFI

    1. `cargo install uniffi_bindgen`

## Adding new structs and functions

See the [UniFFI User Guide](https://mozilla.github.io/uniffi-rs/)

### For pass by value objects

   1. create new rust struct with only fields that are supported UniFFI types
   2. update mapping `bdk.udl` file with new `dictionary`

### For pass by reference values 

   1. create wrapper rust struct/impl with only fields that are `Sync + Send`
   2. update mapping `bdk.udl` file with new `interface`

### Build and test

   1. Use `build.sh` script (TODO do it all in build.rs instead) 
   2. Create tests in `bindings/bdk-kotlin` and/or `bindings/bdk-swift`
   3. Use `test.sh` to run all bindings tests