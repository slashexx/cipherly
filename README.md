# Classic Cipher CLI Tool

A command-line tool for encrypting and decrypting text using various classic ciphers.

## Supported Ciphers

- Caesar Cipher
- Vigenère Cipher
- Playfair Cipher (coming soon)
- Hill Cipher (coming soon)

## Installation

1. Install Rust from https://rustup.rs/
2. Clone this repository
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Caesar Cipher

Encrypt:
```bash
cargo run -- encrypt -c caesar -t "HELLO WORLD" -k 3
```

Decrypt:
```bash
cargo run -- decrypt -c caesar -t "KHOOR ZRUOG" -k 3
```

### Vigenère Cipher

Encrypt:
```bash
cargo run -- encrypt -c vigenere -t "HELLO WORLD" -k "KEY"
```

Decrypt:
```bash
cargo run -- decrypt -c vigenere -t "RIJVS UYVJN" -k "KEY"
```

## Command Line Arguments

- `-c, --cipher`: The cipher to use (caesar, vigenere, playfair, hill)
- `-t, --text`: The text to encrypt/decrypt
- `-k, --key`: The key for the cipher
  - For Caesar: A number (shift amount)
  - For Vigenère: A word or phrase
  - For Playfair: A word or phrase (coming soon)
  - For Hill: A matrix key (coming soon)

## Examples

1. Caesar Cipher with shift 3:
   ```bash
   # Encrypt
   cargo run -- encrypt -c caesar -t "HELLO WORLD" -k 3
   # Output: KHOOR ZRUOG

   # Decrypt
   cargo run -- decrypt -c caesar -t "KHOOR ZRUOG" -k 3
   # Output: HELLO WORLD
   ```

2. Vigenère Cipher with key "KEY":
   ```bash
   # Encrypt
   cargo run -- encrypt -c vigenere -t "HELLO WORLD" -k "KEY"
   # Output: RIJVS UYVJN

   # Decrypt
   cargo run -- decrypt -c vigenere -t "RIJVS UYVJN" -k "KEY"
   # Output: HELLO WORLD
   ```

## Coming Soon

- Playfair Cipher implementation
- Hill Cipher implementation
- Brute force decryption for Caesar Cipher
- Frequency analysis tools 