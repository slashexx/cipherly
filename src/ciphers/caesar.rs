use crate::ciphers::utils::{char_to_index, index_to_char, calculate_frequency, chi_squared};
use std::collections::HashMap;

pub struct CaesarCipher;

impl CaesarCipher {
    pub fn encrypt(text: &str, key: &str) -> Result<String, String> {
        let shift: i32 = key.parse().map_err(|_| "Invalid key: must be a number".to_string())?;
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

    pub fn decrypt(text: &str, key: &str) -> Result<String, String> {
        let shift: i32 = key.parse().map_err(|_| "Invalid key: must be a number".to_string())?;
        Self::encrypt(text, &(-shift).to_string())
    }

    pub fn brute_force(text: &str) -> Vec<(i32, String, f64)> {
        let mut results = Vec::new();
        let freq = calculate_frequency(text);
        
        for shift in 0..26 {
            let decrypted = Self::encrypt(text, &shift.to_string()).unwrap();
            let decrypted_freq = calculate_frequency(&decrypted);
            let chi_squared = chi_squared(&decrypted_freq);
            
            results.push((shift, decrypted, chi_squared));
        }
        
        // Sort by chi-squared (lower is better match to English frequencies)
        results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        results
    }
} 