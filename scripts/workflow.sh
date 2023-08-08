#!/bin/bash

echo -e "\033[0;32mExecuting Workflow locally\033[0m"

python ./scripts/copy_readme.py
cargo check --workspace --verbose
cargo build --workspace --verbose
cargo doc --workspace --verbose
cargo test -F=macros --workspace --verbose
cargo test -F=partial partial --verbose  
cargo clippy --workspace --verbose

cargo fmt --check --all --verbose

python ./scripts/copy_librs.py

cargo llvm-cov --all-features --workspace

echo -e "\033[0;32mExecution Complete\033[0m"
