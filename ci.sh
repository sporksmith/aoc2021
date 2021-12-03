#!/bin/bash

set -euo pipefail

test() {
    name=$1
    actual=$2
    expected=$3
    if [ "$actual" != "$expected" ]
    then
        echo "$name:"
        echo "actual: $actual"
        echo "expect: $expected"
        exit 1
    fi
}

case "${1:-}" in
    "debug" | "")
       bindir=target/debug
       ;;
   "release")
       bindir=target/release
       ;;
esac

unified_test() {
    day=$1
    part=$2
    expected=$3
    test $day$part `RUST_BACKTRACE=1 $bindir/aoc2021 $day$part < inputs/$day.txt` $3
}

unified_test 1 a 1301
unified_test 1 b 1346
unified_test 2 a 1648020