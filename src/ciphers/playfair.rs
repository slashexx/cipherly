use crate::ciphers::utils::{char_to_index, index_to_char};
use std::collections::HashMap;

pub struct PlayfairCipher;

impl PlayfairCipher {
    fn create_key_square(key: &str) -> Vec<Vec<char>> {
        let mut key_square = Vec::with_capacity(5);
        let mut used = HashMap::new();
        let key = key.to_uppercase().replace('J', "I");
        
        for i in 0..5 {
            key_square.push(Vec::with_capacity(5));
            for j in 0..5 {
                key_square[i].push(' ');
            }
        }
        
        let mut row = 0;
        let mut col = 0;
        
        for c in key.chars() {
            if c.is_ascii_alphabetic() && !used.contains_key(&c) {
                used.insert(c, true);
                key_square[row][col] = c;
                col += 1;
                if col == 5 {
                    col = 0;
                    row += 1;
                }
            }
        }
        
        for c in 'A'..='Z' {
            if c != 'J' && !used.contains_key(&c) {
                used.insert(c, true);
                key_square[row][col] = c;
                col += 1;
                if col == 5 {
                    col = 0;
                    row += 1;
                }
            }
        }
        
        key_square
    }
    
    fn find_position(key_square: &Vec<Vec<char>>, c: char) -> (usize, usize) {
        for i in 0..5 {
            for j in 0..5 {
                if key_square[i][j] == c {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }
    
    fn prepare_text(text: &str) -> String {
        let text = text.to_uppercase().replace('J', "I");
        let mut result = String::new();
        let mut chars: Vec<char> = text.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        
        let mut i = 0;
        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == chars[i + 1] {
                result.push(chars[i]);
                result.push('X');
            } else {
                result.push(chars[i]);
                if i + 1 < chars.len() {
                    result.push(chars[i + 1]);
                }
            }
            i += 2;
        }
        
        if result.len() % 2 != 0 {
            result.push('X');
        }
        
        result
    }
    
    pub fn encrypt(text: &str, key: &str) -> Result<String, String> {
        let key_square = Self::create_key_square(key);
        let prepared_text = Self::prepare_text(text);
        let mut result = String::new();
        
        let mut i = 0;
        while i < prepared_text.len() {
            let c1 = prepared_text.chars().nth(i).unwrap();
            let c2 = prepared_text.chars().nth(i + 1).unwrap();
            
            let (row1, col1) = Self::find_position(&key_square, c1);
            let (row2, col2) = Self::find_position(&key_square, c2);
            
            let (new_c1, new_c2) = if row1 == row2 {
                let new_col1 = (col1 + 1) % 5;
                let new_col2 = (col2 + 1) % 5;
                (key_square[row1][new_col1], key_square[row2][new_col2])
            } else if col1 == col2 {
                let new_row1 = (row1 + 1) % 5;
                let new_row2 = (row2 + 1) % 5;
                (key_square[new_row1][col1], key_square[new_row2][col2])
            } else {
                (key_square[row1][col2], key_square[row2][col1])
            };
            
            result.push(new_c1);
            result.push(new_c2);
            i += 2;
        }
        
        Ok(result)
    }
    
    pub fn decrypt(text: &str, key: &str) -> Result<String, String> {
        let key_square = Self::create_key_square(key);
        let mut result = String::new();
        
        let mut i = 0;
        while i < text.len() {
            let c1 = text.chars().nth(i).unwrap();
            let c2 = text.chars().nth(i + 1).unwrap();
            
            let (row1, col1) = Self::find_position(&key_square, c1);
            let (row2, col2) = Self::find_position(&key_square, c2);
            
            let (new_c1, new_c2) = if row1 == row2 {
                let new_col1 = (col1 + 4) % 5;
                let new_col2 = (col2 + 4) % 5;
                (key_square[row1][new_col1], key_square[row2][new_col2])
            } else if col1 == col2 {
                let new_row1 = (row1 + 4) % 5;
                let new_row2 = (row2 + 4) % 5;
                (key_square[new_row1][col1], key_square[new_row2][col2])
            } else {
                (key_square[row1][col2], key_square[row2][col1])
            };
            
            result.push(new_c1);
            result.push(new_c2);
            i += 2;
        }
        
        let mut final_result = String::new();
        let mut i = 0;
        while i < result.len() {
            if i + 2 < result.len() && result.chars().nth(i + 1).unwrap() == 'X' 
               && result.chars().nth(i).unwrap() == result.chars().nth(i + 2).unwrap() {
                final_result.push(result.chars().nth(i).unwrap());
                i += 2;
            } else {
                final_result.push(result.chars().nth(i).unwrap());
                i += 1;
            }
        }
        
        if final_result.ends_with('X') {
            final_result.pop();
        }
        
        Ok(final_result)
    }
} 