if not exist build md build
wasm-pack build --target web --out-name aoe2maps --out-dir build
copy static\* build
