#!/bin/bash

set -e

if [ ! $# -eq 1 ]; then
    echo Need a problem number
    exit -1
fi

NEXT=$1
PREV=$(( $NEXT - 1 ))

cp -r template problem_$NEXT

sed -i "s/problem_xx/problem_$NEXT/" problem_$NEXT/Cargo.toml
sed -i "s/\"problem_$PREV\"/\"problem_$PREV\",\n    \"problem_$NEXT\"/" Cargo.toml

curl -b session=$(cat .session-cookie) https://adventofcode.com/2021/day/$NEXT/input > problem_$NEXT/input.txt

./run_all.sh
