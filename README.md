
UniFFI

1. cargo install uniffi_bindgen 
2. cargo build
3. uniffi-bindgen generate --no-format --out-dir kotlin src/bdk.udl --language kotlin

Setup Android build environment

1. Add Android rust targets

```sh
rustup target add x86_64-apple-darwin x86_64-unknown-linux-gnu x86_64-linux-android aarch64-linux-android armv7-linux-androideabi i686-linux-android
```

2. Set ANDROID_NDK_HOME

```sh
export ANDROID_NDK_HOME=/home/<user>/Android/Sdk/ndk/<NDK version, ie. 21.4.7075529>
```

Setup Swift build environment

1. Install Swift, see ["Download Swift"](https://swift.org/download/) page

Adding new structs and functions

1. Create C safe Rust structs and related functions using safer-ffi

2. Test generated library and `bdk_ffi.h` file with c language tests in `cc/bdk_ffi_test.c`

3. Use `build.sh` and `test.sh` to build c test program and verify functionality and 
   memory de-allocation via `valgrind` 

4. Update the kotlin native interface LibJna.kt in the `bdk-kotlin` `jvm` module to match `bdk_ffi.h`

5. Create kotlin wrapper classes and interfaces as needed

6. Add tests to `bdk-kotlin` `test-fixtures` module 

7. Use `build.sh` and `test.sh` to build and test `bdk-kotlin` `jvm` and `android` modules