#!/bin/bash

set -e

echo "Installing Classic Cipher CLI Tool..."

TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "Cloning repository..."
git clone https://github.com/slashexx/cipherly.git
cd cipherly

echo "Building project..."
cargo build --release

mkdir -p "$HOME/.local/bin"

echo "Installing executable..."
cp target/release/clitool "$HOME/.local/bin/cipher"

chmod +x "$HOME/.local/bin/cipher"

echo "Cleaning up..."
cd "$HOME"
rm -rf "$TEMP_DIR"

if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
  echo "Adding ~/.local/bin to your PATH..."
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
  echo "Please restart your terminal or run 'source ~/.bashrc' to use the tool."
fi

echo "Installation complete! You can now use the 'cipher' command."
echo "Example: cipher encrypt -c caesar -t 'HELLO WORLD' -k 3"