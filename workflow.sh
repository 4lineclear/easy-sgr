#!/bin/bash

echo -e "\033[0;32mExecuting Github workflow locally\033[0m"

python copy_readme.py

cargo build --verbose
cargo doc --verbose
cargo test --all-features --verbose
cargo fmt --all

python copy_librs.py

echo -e "\033[0;32mExecution Complete\033[0m"