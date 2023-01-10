# Rust Template
Clone and initialize this repo for creating a baseline Rust project.

## Initialize Template

    ./prepare.sh && rm ./prepare.sh && git commit -a

## Automated Tests
This project runs automated tests and lints before each
git push action. If you want to trigger tests manually,
run:

    rustup update
    cargo test

## Linting
This project runs automated tests and lints before each
git push action. If you want to trigger linting manually,
run:

    cargo clippy -- -D warnings && cargo fmt -- --check


