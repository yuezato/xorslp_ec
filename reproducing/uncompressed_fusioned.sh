#!/bin/bash

set -eu

blocks=( 64 128 256 512 1024 2048 4096 )

for i in "${blocks[@]}"; do
    tmp=$i
    tmp+="block"
    eval "block=$tmp"
    echo "<$block>"
    cargo build --release --features $block
    ../target/release/xorslp_ec --no-compress --optimize-level Fusion --enc-dec
    echo "</$block>"
    echo ""
done
