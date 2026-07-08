build:
    cargo build

run:
    cargo run

check:
    cargo check

fmt:
    cargo fmt

clippy:
    cargo clippy --all-targets --all-features -- -D warnings
