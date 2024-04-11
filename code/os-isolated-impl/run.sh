#!/usr/bin/env bash
cargo build --release
sudo cset shield --cpu 3 --kthread on > /dev/null
sudo cset shield --exec ./target/release/os-isolated-impl --user graukolos --group graukolos
sudo cset shield --reset > /dev/null
