from rust:1.50

arg user
arg group

# create user
run useradd -ms /bin/bash -u $user -g $group docker-build
user $user:$group
workdir /home/docker-build

# get dependencies
run cargo install wasm-pack

# build dependencies
run mkdir aoe2maps
workdir aoe2maps
copy Cargo.* ./
run mkdir -p src && touch src/lib.rs
run wasm-pack build --release --target web --out-name aoe2maps --out-dir build

# build project
copy . .
cmd wasm-pack build --release --target web --out-name aoe2maps --out-dir build
