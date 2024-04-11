#!/usr/bin/env bash

cargo build --release
# requires the cpuset package to be installed
sudo cset shield --cpu 3 --kthread on
sudo cset shield --shield --verbose
sudo cset shield --exec "cargo run --release" --user graukolos --group graukolos
sudo cset shield --reset