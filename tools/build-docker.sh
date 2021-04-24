#!/usr/env/bin sh

rm -Rf build
mkdir build 
docker build -t aoe2maps -f tools/Dockerfile --build-arg user=$(id -u) --build-arg group=$(id -g) .
docker run -v $(pwd)/build:/aoe2maps/build aoe2maps
cp static/* build
