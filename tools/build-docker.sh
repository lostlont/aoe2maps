#!/usr/env/bin sh

rm -Rf build
mkdir build
docker build -t aoe2maps -f tools/Dockerfile --build-arg user=$(id -u) --build-arg group=$(id -g) .
docker run --rm -v /$(pwd)/build:/home/docker-build/aoe2maps/build aoe2maps
cp -R static/* build
