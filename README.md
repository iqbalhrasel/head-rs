# head-rs

A simple implementation of the Unix `head` command written in Rust.

`head-rs` reads the beginning of a file and prints the first **N lines** to standard output.

## Features

- Read from a file
- Print the first 10 lines by default
- Customize the number of lines with `-n`
- Uses `BufReader` for efficient file reading
- Basic command-line argument handling
- Written entirely in Rust

## Installation

Clone the repository:

```bash
git clone https://github.com/iqbalhrasel/head-rs.git
cd head-rs
```

## Why I Built This

This project is part of my Rust CLI learning journey.

Implementing familiar Unix utilities from scratch is a useful way to understand Rust's:

- File I/O
- BufReader
- Iterators
- Command-line arguments
- Error handling
- Standard input/output
- Ownership and borrowing
