#!/usr/bin/env bash
cargo clippy
nohup cargo run --release  --bin web3_quick