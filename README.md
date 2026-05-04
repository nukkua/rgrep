# rgrep

Grep del temu
A lightweight grep-like CLI tool written in Rust.

## Requirements

- [Rust](https://rustup.rs/) 1.70 or later

## Installation

### From source

```bash
git clone https://github.com/nukkua/rgrep
cd rgrep
cargo build --release
```

The binary will be at `./target/release/rgrep`.

### Install globally with Cargo

```bash
cargo install --path .
```

After this, `rgrep` is available anywhere in your terminal.

## Usage

```
rgrep [OPTIONS] <PATTERN> <FILE>...
```

### Options

| Flag | Long form | Description |
|------|-----------|-------------|
| `-i` | `--insensitive` | Case-insensitive search |
| `-n` | `--lines` | Show line numbers |

### Examples

Search for a pattern in a file:
```bash
rgrep "hello" src/main.rs
```

Case-insensitive search across multiple files:
```bash
rgrep -i "error" src/main.rs src/cli.rs
```

Show line numbers:
```bash
rgrep -n "fn main" src/main.rs
```

Combine flags:
```bash
rgrep -i -n "todo" src/main.rs src/matcher.rs
```

## Project Structure

```
rgrep/
├── Cargo.toml
└── src/
    ├── main.rs       # Entry point and orchestration
    ├── cli.rs        # Argument parsing and Config
    ├── matcher.rs    # Pattern matching logic
    └── error.rs      # Error types
```

## Error Handling

`rgrep` exits with code `1` and prints to `stderr` on:
- Missing pattern or files
- Unknown flags
- Unreadable files (skips the file and continues with the rest)
