#!/bin/bash

set -e

cargo build --release

for i in problem*; do
    echo $i
    echo ----------------
    time target/release/$i $i/input.txt
    echo ----------------
done
