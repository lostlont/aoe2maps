rd build /s /q
md build
wasm-pack build --release --target web --out-name aoe2maps --out-dir build
copy static\* build
