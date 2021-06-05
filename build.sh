cargo build
cargo test --features c-headers -- generate_headers
cc main.c -o main -L target/debug -l bdk_ffi -l pthread -l dl -l m