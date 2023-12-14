#!/bin/bash

git pull

cd ./active_oberon_compiler
cargo test

echo "Building release version"
cargo build --release
strip ./target/release/active_oberon_compiler

cd ..
cp ./active_oberon_compiler/target/release/active_oberon_compiler ./aoc

echo "Done ...  Use 'aoc' to use compiler"


