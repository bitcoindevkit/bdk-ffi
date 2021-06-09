cargo build
cargo test --features c-headers -- generate_headers
cc main.c -o main -L target/debug -l bdk_ffi -l pthread -l dl -l m
./main 

# jvm
mkdir -p jvm/build/jniLibs/x86_64_linux
cp target/debug/libbdk_ffi.so jvm/build/jniLibs/x86_64_linux
export LD_LIBRARY_PATH=`pwd`/target/debug
