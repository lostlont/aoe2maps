from rust:1.50

# get dependencies
run cargo install wasm-pack

# build dependencies
workdir /aoe2maps
copy Cargo.* ./
run mkdir src && touch src/lib.rs
run wasm-pack build --release --target web --out-name aoe2maps --out-dir build

# build project
copy . .
cmd wasm-pack build --release --target web --out-name aoe2maps --out-dir build
