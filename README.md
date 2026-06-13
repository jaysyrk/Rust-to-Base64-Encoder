# Rust Base64 Encoder/Decoder

A high-performance Base64 encoder and decoder written entirely from scratch in Rust. This tool was built as a learning exercise to understand byte-level manipulation and bitwise operations without relying on any external dependencies or crates.

It features a dual-target architecture:
- **Core Library (`lib.rs`)**: Contains the raw algorithmic logic (bit shifting and masking).
- **CLI Wrapper (`main.rs`)**: A lightweight command-line interface that allows you to easily encode or decode strings and files directly from your terminal.

## Installation

To install this tool globally on your system, ensure you have Rust and Cargo installed, then run the following command from the root of this project:

```bash
cargo install --path .
```

This will compile the binary and add `rust-base64` to your path.

## Usage

You can use `rust-base64` to encode or decode either text snippets or full files.

### Encoding
To encode a file, provide the `encode` command followed by the file path:
```bash
rust-base64 encode /path/to/file.txt
```

*(Note: It will print the Base64 output directly to your terminal. You can pipe this into a file if you wish: `rust-base64 encode data.txt > encoded.txt`)*

### Decoding
To decode a Base64 file back into its original text, use the `decode` command:
```bash
rust-base64 decode encoded.txt
```
