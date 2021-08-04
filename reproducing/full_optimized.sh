#!/bin/bash

set -eu

blocks=( 64 128 256 512 1024 2048 4096 )

echo "<GREEDY SCHEDULING>"
for i in "${blocks[@]}"; do
    tmp=$i
    tmp+="block"
    eval "block=$tmp"
    echo "<$block>"    
    set -x
    cargo build --release --features "$block bottomup_sched"
    { set +x ;} 2> /dev/null
    ../target/release/xorslp_ec --enc-dec
    echo "</$block>"
    echo ""
done
echo "</GREEDY SCHEDULING>"

echo ""
echo ""
echo "<DFS SCHEDULING>"
for i in "${blocks[@]}"; do
    tmp=$i
    tmp+="block"
    eval "block=$tmp"
    echo "<$block>"
    set -x
    cargo build --release --features "$block dfs_sched"
    { set +x ;} 2> /dev/null
    ../target/release/xorslp_ec --enc-dec
    echo "</$block>"
    echo ""    
done
echo "</DFS SCHEDULING>"
