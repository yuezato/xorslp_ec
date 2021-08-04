#!/bin/bash

XORSLP_EC=../target/release/xorslp_ec

set -eu

cargo build --release --features 2048block

for i in 4 3 2; do
    echo "< RS(8, $i) >"
    $XORSLP_EC --data-block 8 --parity-block $i --enc-dec
    echo "</ RS(8, $i) >"

    echo ""
    echo "< RS(9, $i) >"
    $XORSLP_EC --data-block 9 --parity-block $i --enc-dec
    echo "</ RS(9, $i) >"

    echo ""
    echo "< RS(10, $i) >"
    $XORSLP_EC --data-block 10 --parity-block $i --enc-dec
    echo "</ RS(10, $i) >"

    echo ""
done
