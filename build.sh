#!/bin/bash

# build rust
cargo build --target wasm32-unknown-unknown --release

# bind it, with wasm-bindgen CLI (available here https://crates.io/crates/wasm-bindgen-cli).
# be careful, wasm-bindgen CLI & wasm-bindgen versions have to match, you may have to
# update wasm-bindgen to make it work
wasm-bindgen ./target/wasm32-unknown-unknown/release/wkrakmi.wasm --out-dir build --no-typescript --no-modules

# strip it if you want
# wasm-strip build/wasm_crackme_bg.wasm

# launch http serv
basic-http-server build/
