

Adding new structs and functions

1. Create C safe Rust structs and related functions using safer-ffi

2. Test generated library and `bdk_ffi.h` file with c language tests in `cc/bdk_ffi_test.c`

3. Use `build.sh` and `test.sh` to build c test program and verify functionality and 
   memory de-allocation via `valgrind` 

4. Update the kotlin native interface LibJna.kt in the `bdk-kotlin` `jvm` module to match `bdk_ffi.h`

5. Create kotlin wrapper classes and interfaces as needed

6. Add tests to `bdk-kotlin` `test-fixtures` module 

7. Use `build.sh` and `test.sh` to build and test `bdk-kotlin` `jvm` and `android` modules