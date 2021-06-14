# rust
cargo build
cargo test --features c-headers -- generate_headers

# cc
export LD_LIBRARY_PATH=`pwd`/target/debug
cc cc/bdk_ffi_test.c -o cc/bdk_ffi_test -L target/debug -l bdk_ffi -l pthread -l dl -l m
valgrind --leak-check=full cc/bdk_ffi_test
#cc/bdk_ffi_test

# bdk-kotlin
mkdir -p bdk-kotlin/jar/libs/x86_64_linux
cp target/debug/libbdk_ffi.so bdk-kotlin/jar/libs/x86_64_linux
(cd bdk-kotlin && gradle test)
