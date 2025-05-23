use clap::{Parser, Subcommand};
use thiserror::Error;

mod ciphers;

use ciphers::{CaesarCipher, VigenereCipher, PlayfairCipher, HillCipher};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[arg(short, long)]
    Encrypt {
        #[arg(short, long)]
        cipher: String,
        
        #[arg(short, long)]
        text: String,
        
        #[arg(short, long)]
        key: String,
    },
    
    #[arg(short, long)]
    Decrypt {
        #[arg(short, long)]
        cipher: String,
        
        #[arg(short, long)]
        text: String,
        
        #[arg(short, long)]
        key: String,
    },
    
    #[arg(short, long)]
    BruteForce {
        #[arg(short, long)]
        text: String,
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
        Commands::BruteForce { text } => {
            let results = CaesarCipher::brute_force(text);
            println!("Possible decryptions (sorted by likelihood):");
            for (shift, decrypted, chi_squared) in results {
                println!("Shift {}: {} (chi-squared: {:.4})", shift, decrypted, chi_squared);
            }
        }
    }
}

fn encrypt(cipher: &str, text: &str, key: &str) -> Result<String, String> {
    match cipher.to_lowercase().as_str() {
        "caesar" => CaesarCipher::encrypt(text, key),
        "vigenere" => VigenereCipher::encrypt(text, key),
        "playfair" => PlayfairCipher::encrypt(text, key),
        "hill" => HillCipher::encrypt(text, key),
        _ => Err(format!("Invalid cipher type: {}", cipher)),
    }
}

fn decrypt(cipher: &str, text: &str, key: &str) -> Result<String, String> {
    match cipher.to_lowercase().as_str() {
        "caesar" => CaesarCipher::decrypt(text, key),
        "vigenere" => VigenereCipher::decrypt(text, key),
        "playfair" => PlayfairCipher::decrypt(text, key),
        "hill" => HillCipher::decrypt(text, key),
        _ => Err(format!("Invalid cipher type: {}", cipher)),
    }
} 