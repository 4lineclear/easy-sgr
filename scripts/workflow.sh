#!/bin/bash

echo -e "\033[0;32mExecuting Github workflow locally\033[0m"

python ./scripts/copy_readme.py

cargo check
cargo build --verbose
cargo doc --verbose
cargo test --all-features --verbose
cargo fmt --check --all
cargo clippy

python ./scripts/copy_librs.py

cargo llvm-cov --all-features --workspace

echo -e "\033[0;32mExecution Complete\033[0m"