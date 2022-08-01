#!/usr/bin/env bash

set -eu

cargo +nightly contract build --manifest-path accumulator/Cargo.toml --skip-linting
cargo +nightly contract build --manifest-path adder/Cargo.toml --skip-linting
cargo +nightly contract build --manifest-path subber/Cargo.toml --skip-linting
cargo +nightly contract build --manifest-path counter/Cargo.toml --skip-linting
cargo +nightly contract build --skip-linting
