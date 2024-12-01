# AdventOfCode2024

GitHub Action Status: ![rust](https://github.com/alex-precosky/AdventOfCode2024/actions/workflows/rust.yml/badge.svg)

My solutions to advent of code 2024. I'll by trying to do this in rust this year.

Development was done in Windows Subsystem for Linux 2 on Windows 10.

# Requirements

A rust toolchain of at least version 1.82. See
https://www.rust-lang.org/tools/install for hints on installation.

Any modern Linux, macOS, or Windows OS ought work.

# Run

Each solution is in a cargo project named after what day's problem that solution
is for. To run the day 1 solution, simply run:

```
cargo run day01
```

# Testing

From the project directory, run the unit tests for a day with:

```
cargo test day01
```
