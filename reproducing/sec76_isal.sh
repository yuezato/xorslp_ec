#!/bin/bash

set -eu

function compile() {
    rm -f bench_isal;

    # gcc -O3 -DDATA_BLOCK=$1 -DPARITY_BLOCK=$2 -DDATA_SIZE=10000000 bench_isal.c -Iinclude -lisal -o bench_isal; # for Mac

    gcc -O3 -DDATA_BLOCK=$1 -DPARITY_BLOCK=$2 -DDATA_SIZE=10000000 bench_isal.c -Iinclude -L.libs -lisal -lm -Wl,-rpath .libs -o bench_isal # for Linux
}

for i in 4 3 2; do
    echo "< RS(8, $i) >"
    compile 8 $i
    ./bench_isal
    echo "</ RS(8, $i) >"

    echo ""
    echo "< RS(9, $i) >"
    compile 9 $i
    ./bench_isal
    echo "</ RS(9, $i) >"

    echo ""
    echo "< RS(10, $i) >"
    compile 10 $i
    ./bench_isal    
    echo "</ RS(10, $i) >"

    echo ""
done
