# Advent of Code - 2025

This project is tracking my progress on the 2025 Advent of Code challenge, with me going for a full rust implementation (learning the language as I go).

# Configuring

- [Install Rust](https://www.rust-lang.org/tools/install)

# Building

This project uses cargo workspaces, so to build any project you simply run the following within the root project directory:

`$ cargo build`

# Testing

From the root project directory:

`$ cargo test`

# Running

Executables can be found in `./target/debug/`. All solutions read their inputs from standard input, so you'll need to cat
the input string into the program. For example:

`cat aoc-1/inputs/puzzle.txt | target/debug/aoc_1`
