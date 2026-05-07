#!/bin/bash

cargo +nightly miri nextest run --no-fail-fast
