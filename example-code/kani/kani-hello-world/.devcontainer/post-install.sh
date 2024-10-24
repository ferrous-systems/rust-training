#!/bin/bash
sudo apt update
sudo apt install -y build-essential
sudo apt install -y universal-ctags
cargo install --locked kani-verifier
cargo kani setup
