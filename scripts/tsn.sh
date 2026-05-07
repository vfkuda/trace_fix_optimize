#!/bin/bash

## ThreadSanitizer

#cargo clean; \
RUSTFLAGS="-Zsanitizer=thread" \
RUSTDOCFLAGS="-Zsanitizer=thread" \
cargo +nightly test -Zbuild-std --target aarch64-apple-darwin --test integration
