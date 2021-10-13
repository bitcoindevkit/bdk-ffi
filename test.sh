#!/usr/bin/env bash
set -eo pipefail

# functions

## help
help()
{
   # Display Help
   echo "Test bdk-uniffi and related libraries."
   echo
   echo "Syntax: build [-a|h|k|v]"
   echo "options:"
   echo "-h     Print this Help."
   echo "-k     Kotlin tests."
   echo
}

test_kotlin() {
  (cd bindings/bdk-kotlin && ./gradlew test -Djna.debug_load=true)
}

if [ $1 = "-h" ]
then
  help
else
  cargo test

  # optional tests
  while [ -n "$1" ]; do # while loop starts
    case "$1" in
      -h) help ;;
      -k) test_kotlin ;;
      *) echo "Option $1 not recognized" ;;
    esac
    shift
  done
fi
