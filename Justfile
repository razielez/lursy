set positional-arguments
set dotenv-load := true

help:
    @just --list --unsorted

clean:
    cargo clean

build:
    cargo build
alias b := build

run *args:
    cargo run -- "$@"
alias r := run

fmt:
    cargo fmt

release:
    cargo build --release

install:
    cargo install --path .
alias i := install
