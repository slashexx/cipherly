# Classic Cipher CLI Tool

A command-line tool for encrypting and decrypting text using various classic ciphers.

## Supported Ciphers

- Caesar Cipher
- Vigenère Cipher
- Playfair Cipher
- Hill Cipher

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

Brute Force Decrypt:
```bash
cargo run -- brute-force -t "KHOOR ZRUOG"
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

### Playfair Cipher

Encrypt:
```bash
cargo run -- encrypt -c playfair -t "HELLO WORLD" -k "KEYWORD"
```

Decrypt:
```bash
cargo run -- decrypt -c playfair -t "GDXXN QNKZU" -k "KEYWORD"
```

### Hill Cipher

Encrypt:
```bash
cargo run -- encrypt -c hill -t "HELLO" -k "9 4 5 7"
```

Decrypt:
```bash
cargo run -- decrypt -c hill -t "ZNKYY" -k "9 4 5 7"
```

## Command Line Arguments

- `-c, --cipher`: The cipher to use (caesar, vigenere, playfair, hill)
- `-t, --text`: The text to encrypt/decrypt
- `-k, --key`: The key for the cipher
  - For Caesar: A number (shift amount)
  - For Vigenère: A word or phrase
  - For Playfair: A word or phrase
  - For Hill: Space-separated numbers for the matrix (e.g., "9 4 5 7" for a 2x2 matrix)

## Examples

1. Caesar Cipher with shift 3:
   ```bash
   # Encrypt
   cargo run -- encrypt -c caesar -t "HELLO WORLD" -k 3
   # Output: KHOOR ZRUOG

   # Decrypt
   cargo run -- decrypt -c caesar -t "KHOOR ZRUOG" -k 3
   # Output: HELLO WORLD

   # Brute Force
   cargo run -- brute-force -t "KHOOR ZRUOG"
   # Output: Shows all possible shifts sorted by likelihood
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

3. Playfair Cipher with key "KEYWORD":
   ```bash
   # Encrypt
   cargo run -- encrypt -c playfair -t "HELLO WORLD" -k "KEYWORD"
   # Output: GDXXN QNKZU

   # Decrypt
   cargo run -- decrypt -c playfair -t "GDXXN QNKZU" -k "KEYWORD"
   # Output: HELLO WORLD
   ```

4. Hill Cipher with 2x2 matrix key:
   ```bash
   # Encrypt
   cargo run -- encrypt -c hill -t "HELLO" -k "9 4 5 7"
   # Output: ZNKYY

   # Decrypt
   cargo run -- decrypt -c hill -t "ZNKYY" -k "9 4 5 7"
   # Output: HELLO
   ```

## Features

- Caesar Cipher with brute force decryption using frequency analysis
- Vigenère Cipher with keyword-based encryption/decryption
- Playfair Cipher with 5x5 key square
- Hill Cipher with 2x2 and 3x3 matrix support
- Frequency analysis tools for cryptanalysis 