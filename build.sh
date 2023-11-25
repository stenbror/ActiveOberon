#!/bin/sh

if [ ! -d build ]
then
    mkdir build
fi

cd build
cmake ..
make -j 4

if [ -f ActiveOberon ]
then
    strip ActiveOberon
    ctest
    cd ..
fi

