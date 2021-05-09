from rust:1.52

arg user
arg group

# create user
run groupadd -fg $group docker-build
run useradd -ms /bin/bash -u $user -g $group docker-build
user $user:$group
workdir /home/docker-build

# get dependencies
run cargo install wasm-pack

# build dependencies
run mkdir aoe2maps
workdir aoe2maps
copy --chown=$user:$group Cargo.* ./
run mkdir -p src && touch src/lib.rs
run wasm-pack build --release --target web --out-name aoe2maps --out-dir build
run rm build/*

# build project
copy --chown=$user:$group . .
cmd wasm-pack build --release --target web --out-name aoe2maps --out-dir build
