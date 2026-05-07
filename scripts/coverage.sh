#!/bin/bash

###
### required:
###
# rustup component add llvm-tools-preview
# cargo install cargo-llvm-cov
###

RUSTFLAGS="--remap-path-prefix=$(pwd)=." \
cargo llvm-cov --html --open --ignore-filename-regex "benches/|tests/|generated/"
