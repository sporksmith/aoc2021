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
    test $day$part `$bindir/aoc2020 $day$part < inputs/day$day` $3
}

# Example:
# unified_test 7 a 265
