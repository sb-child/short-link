#!/bin/sh

mkdir -p build

# RUSTFLAGS="-C target-feature=-crt-static" 
cargo build --bin short_link -r --target=x86_64-unknown-linux-musl || exit 1

cp target/x86_64-unknown-linux-musl/release/short_link build/short_link-amd64 || exit 1

# cargo build -r --target=x86_64-pc-windows-gnu || exit 1

# cp target/x86_64-pc-windows-gnu/release/short_link.exe build/short_link-amd64.exe || exit 1
