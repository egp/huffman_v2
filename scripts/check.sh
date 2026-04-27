#!/bin/zsh

cargo fmt &&
cargo clippy -- -D warnings &&
cargo check &&
cargo doc &&
cargo clean &&
cargo test
