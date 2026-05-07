#!/bin/bash

#AddressSanitizer

#cargo clean; \ 
RUSTFLAGS="-Zsanitizer=address" \
RUSTDOCFLAGS="-Zsanitizer=address" \
ASAN_OPTIONS="detect_leaks=1" \
cargo +nightly test -Zbuild-std --target aarch64-apple-darwin --test integration