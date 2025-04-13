use crate::ciphers::utils::{char_to_index, index_to_char};

pub struct VigenereCipher;

impl VigenereCipher {
    pub fn encrypt(text: &str, key: &str) -> Result<String, String> {
        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Key must contain only letters".to_string());
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

    pub fn decrypt(text: &str, key: &str) -> Result<String, String> {
        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Key must contain only letters".to_string());
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
} 