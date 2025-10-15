# Rudd - Rust Directory Delta

**Rudd** is a CLI tool to visualize differences (delta) between two directories. It shows which files are unique to each directory and which are common.

## Problem description

When I do old style backups of photos from my laptop to my external SSD-disc, it's tricky to remember which directories is already copied and which ones are not. This tool solves that problem.

Using `$ diff --recursive --brief directory1 directory2` will also solve it, but what's the fun in that?

## Features

- Get delta in files between two directories
- Colorful, east-to-read/easy-to-understand output
- Verbose mode

## Usage

```sh
rudd <MAIN_DIR> <COMPARE_DIR> [--verbose] [--diff-only]

# Detailed help
rudd -h
```

## Building

```bash
cargo build --release

# Local build and run when not installed
cargo run -- [OPTIONS] <MAIN_DIR> <COMPARE_DIR>
```

## Installation

```bash
git clone <repo>
cd rudd
cargo install --path .
```

## Testing

```bash
cargo test
```