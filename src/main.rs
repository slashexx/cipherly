use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt text using a specified cipher
    Encrypt {
        /// The cipher to use (caesar, vigenere, playfair, hill)
        #[arg(short, long)]
        cipher: String,
        
        /// The text to encrypt
        #[arg(short, long)]
        text: String,
        
        /// The key for the cipher
        #[arg(short, long)]
        key: String,
    },
    
    /// Decrypt text using a specified cipher
    Decrypt {
        /// The cipher to use (caesar, vigenere, playfair, hill)
        #[arg(short, long)]
        cipher: String,
        
        /// The text to decrypt
        #[arg(short, long)]
        text: String,
        
        /// The key for the cipher
        #[arg(short, long)]
        key: String,
    },
}

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Invalid cipher type: {0}")]
    InvalidCipher(String),
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { cipher, text, key } => {
            match encrypt(cipher, text, key) {
                Ok(result) => println!("Encrypted text: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Decrypt { cipher, text, key } => {
            match decrypt(cipher, text, key) {
                Ok(result) => println!("Decrypted text: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

fn encrypt(cipher: &str, text: &str, key: &str) -> Result<String, CipherError> {
    match cipher.to_lowercase().as_str() {
        "caesar" => caesar_encrypt(text, key),
        "vigenere" => vigenere_encrypt(text, key),
        "playfair" => playfair_encrypt(text, key),
        "hill" => hill_encrypt(text, key),
        _ => Err(CipherError::InvalidCipher(cipher.to_string())),
    }
}

fn decrypt(cipher: &str, text: &str, key: &str) -> Result<String, CipherError> {
    match cipher.to_lowercase().as_str() {
        "caesar" => caesar_decrypt(text, key),
        "vigenere" => vigenere_decrypt(text, key),
        "playfair" => playfair_decrypt(text, key),
        "hill" => hill_decrypt(text, key),
        _ => Err(CipherError::InvalidCipher(cipher.to_string())),
    }
}

// Caesar Cipher implementation
fn caesar_encrypt(text: &str, key: &str) -> Result<String, CipherError> {
    let shift: i32 = key.parse().map_err(|_| CipherError::InvalidKey(key.to_string()))?;
    let shift = shift % 26;
    
    Ok(text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
                let c = c as i32 - base as i32;
                let shifted = (c + shift).rem_euclid(26);
                (base as i32 + shifted) as u8 as char
            } else {
                c
            }
        })
        .collect())
}

fn caesar_decrypt(text: &str, key: &str) -> Result<String, CipherError> {
    let shift: i32 = key.parse().map_err(|_| CipherError::InvalidKey(key.to_string()))?;
    caesar_encrypt(text, &(-shift).to_string())
}

// Vigenère Cipher implementation
fn vigenere_encrypt(text: &str, key: &str) -> Result<String, CipherError> {
    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(CipherError::InvalidKey("Key must contain only letters".to_string()));
    }
    
    let key = key.to_uppercase();
    let key_chars: Vec<char> = key.chars().collect();
    let mut key_index = 0;
    
    Ok(text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
                let c = c as i32 - base as i32;
                let k = key_chars[key_index % key_chars.len()] as i32 - 'A' as i32;
                let shifted = (c + k).rem_euclid(26);
                key_index += 1;
                (base as i32 + shifted) as u8 as char
            } else {
                c
            }
        })
        .collect())
}

fn vigenere_decrypt(text: &str, key: &str) -> Result<String, CipherError> {
    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(CipherError::InvalidKey("Key must contain only letters".to_string()));
    }
    
    let key = key.to_uppercase();
    let key_chars: Vec<char> = key.chars().collect();
    let mut key_index = 0;
    
    Ok(text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
                let c = c as i32 - base as i32;
                let k = key_chars[key_index % key_chars.len()] as i32 - 'A' as i32;
                let shifted = (c - k).rem_euclid(26);
                key_index += 1;
                (base as i32 + shifted) as u8 as char
            } else {
                c
            }
        })
        .collect())
}

// Placeholder functions for Playfair and Hill ciphers
fn playfair_encrypt(_text: &str, _key: &str) -> Result<String, CipherError> {
    Err(CipherError::InvalidCipher("Playfair cipher not yet implemented".to_string()))
}

fn playfair_decrypt(_text: &str, _key: &str) -> Result<String, CipherError> {
    Err(CipherError::InvalidCipher("Playfair cipher not yet implemented".to_string()))
}

fn hill_encrypt(_text: &str, _key: &str) -> Result<String, CipherError> {
    Err(CipherError::InvalidCipher("Hill cipher not yet implemented".to_string()))
}

fn hill_decrypt(_text: &str, _key: &str) -> Result<String, CipherError> {
    Err(CipherError::InvalidCipher("Hill cipher not yet implemented".to_string()))
} 