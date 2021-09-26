#!/usr/bin/env bash
set -eo pipefail

# functions

## help
help()
{
   # Display Help
   echo "Test bdk-ffi and related libraries."
   echo
   echo "Syntax: build [-a|h|k|v]"
   echo "options:"
   echo "-a     Android aar tests."
   echo "-h     Print this Help."
   echo "-k     JVM jar tests."
   echo "-v     Valgrind tests."
   echo
}

# rust
c_headers() {
  cargo test --features c-headers -- generate_headers
}

# cc
test_c() {
  export LD_LIBRARY_PATH=`pwd`/target/debug
  cc/bdk_ffi_test
}

test_valgrind() {
  valgrind --leak-check=full --show-leak-kinds=all cc/bdk_ffi_test
}

test_kotlin() {
  (cd bdk-kotlin && ./gradlew test)
}

test_android() {
  (cd bdk-kotlin && ./gradlew :android:connectedDebugAndroidTest)
}

if [ $1 = "-h" ]
then
  help
else
  c_headers
  test_c
    
  # optional tests
  while [ -n "$1" ]; do # while loop starts
    case "$1" in          
      -a) test_android ;;
      -h) help ;;
      -k) test_kotlin ;;
      -v) test_valgrind ;;
      *) echo "Option $1 not recognized" ;;
    esac
    shift
  done
fi