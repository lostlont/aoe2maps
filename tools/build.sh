#!/usr/env/bin sh

rm -Rf build
mkdir build 
wasm-pack build --release --target web --out-name aoe2maps --out-dir build
cp static/* build
