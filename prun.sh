#!/bin/bash
export RUSTFLAGS="-C target-cpu=native"
set -euxo pipefail
IFS=$'\n\t'
SLEEP_SEC=2
cd "$(dirname "$0")"

rm ./target/*.csv

cargo build --release --bin proxide-spsc
# cargo build --release --bin kanal-async

sleep $SLEEP_SEC
./target/release/proxide-spsc | tee target/proxide-spsc.csv

./plot.py target/*.csv

echo "Test Environment:"
uname -srvp
rustc --version
go version