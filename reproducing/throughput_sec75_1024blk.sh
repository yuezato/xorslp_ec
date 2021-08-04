#!/bin/bash

XORSLP_EC=../target/release/xorslp_ec

set -eu

cargo build --release --features 1024block

echo "< P >"
$XORSLP_EC --no-compress --optimize-level Nooptim --enc-dec
echo "</ P >"

echo ""
echo "< Co(P) >"
$XORSLP_EC --optimize-level Nooptim --enc-dec
echo "</ Co(P) >"

echo ""
echo "< Fu(Co(P)) >"
$XORSLP_EC --optimize-level Fusion --enc-dec
echo "</ Fu(Co(P)) >"

echo ""
echo "< Dfs(Fu(Co(P))) >"
$XORSLP_EC --enc-dec
echo "</ Dfs(Fu(Co(P))) >"
