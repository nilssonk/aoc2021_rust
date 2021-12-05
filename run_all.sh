#!/bin/bash

set -e

cargo build --release

for i in problem*; do
    cargo run --release --bin $i $i/input.txt
done
