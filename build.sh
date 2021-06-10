# rust
cargo build
cargo test --features c-headers -- generate_headers
export LD_LIBRARY_PATH=`pwd`/target/debug

# cc
cc bdk_ffi_test.c -o bdk_ffi_test -L target/debug -l bdk_ffi -l pthread -l dl -l m
#valgrind --leak-check=full ./bdk_ffi_test
./bdk_ffi_test

# jvm
mkdir -p jvm/build/jniLibs/x86_64_linux
cp target/debug/libbdk_ffi.so jvm/build/jniLibs/x86_64_linux
