#!/bin/bash

rustup default stable

echo "Adding wasm target"
rustup target add wasm32-unknown-unknown

echo "Installing trunk"
cargo install trunk

echo "Building site"
trunk build --release
