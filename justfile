# List available commands in the just file
help:
    @just --list --unsorted --list-prefix '   ' --list-heading $'Restful API workspace\n'
    @echo ' '
    @just --list --unsorted --list-prefix '   ' --list-heading $'  Book service\n' --justfile crates/book_service/justfile

# Run git add, commit, push
push message:
    git add . && git commit -m "{{message}}" && git push 

# Run lints on the workspace members (cargo fmt and clippy)
lint:
    cargo +nightly fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings

# format the rust files
format:
    cargo +nightly fmt

# Run cargo check on the workspace members
check:
    cargo check --workspace

# Run cargo build on the workspace members
build:
    cargo build --workspace --all-targets 

# Run cargo clean on the workspace members
clean:
    cargo clean

# Run cargo test on the workspace members
test:
    cargo test --workspace

# Forward to the BOOK-SERVICE
mod book "crates/book_service"