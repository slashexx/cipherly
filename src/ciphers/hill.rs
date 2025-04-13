use crate::ciphers::utils::{char_to_index, index_to_char};
use std::collections::HashMap;

pub struct HillCipher;

impl HillCipher {
    fn parse_key(key: &str) -> Result<Vec<Vec<i32>>, String> {
        let numbers: Vec<i32> = key
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| "Invalid key format".to_string())?;
            
        let n = (numbers.len() as f64).sqrt() as usize;
        if n * n != numbers.len() {
            return Err("Key must be a perfect square".to_string());
        }
        
        let mut matrix = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                row.push(numbers[i * n + j] % 26);
            }
            matrix.push(row);
        }
        
        Ok(matrix)
    }
    
    fn matrix_multiply(matrix: &Vec<Vec<i32>>, vector: &Vec<i32>) -> Vec<i32> {
        let n = matrix.len();
        let mut result = vec![0; n];
        
        for i in 0..n {
            for j in 0..n {
                result[i] = (result[i] + matrix[i][j] * vector[j]) % 26;
            }
        }
        
        result
    }
    
    fn mod_inverse(a: i32, m: i32) -> Option<i32> {
        let mut t = (0, 1);
        let mut r = (m, a);
        
        while r.1 != 0 {
            let q = r.0 / r.1;
            t = (t.1, t.0 - q * t.1);
            r = (r.1, r.0 - q * r.1);
        }
        
        if r.0 > 1 {
            None
        } else {
            Some((t.0 % m + m) % m)
        }
    }
    
    fn matrix_inverse(matrix: &Vec<Vec<i32>>) -> Result<Vec<Vec<i32>>, String> {
        let n = matrix.len();
        let mut det = 0;
        
        if n == 2 {
            det = (matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]) % 26;
        } else if n == 3 {
            det = (matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
                - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
                + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0])) % 26;
        } else {
            return Err("Only 2x2 and 3x3 matrices are supported".to_string());
        }
        
        let det = (det + 26) % 26;
        let det_inv = Self::mod_inverse(det, 26)
            .ok_or_else(|| "Matrix is not invertible".to_string())?;
            
        let mut inverse = vec![vec![0; n]; n];
        
        if n == 2 {
            inverse[0][0] = (matrix[1][1] * det_inv) % 26;
            inverse[0][1] = (-matrix[0][1] * det_inv) % 26;
            inverse[1][0] = (-matrix[1][0] * det_inv) % 26;
            inverse[1][1] = (matrix[0][0] * det_inv) % 26;
        } else if n == 3 {
            for i in 0..3 {
                for j in 0..3 {
                    let mut cofactor = Vec::new();
                    for r in 0..3 {
                        if r != i {
                            let mut row = Vec::new();
                            for c in 0..3 {
                                if c != j {
                                    row.push(matrix[r][c]);
                                }
                            }
                            cofactor.push(row);
                        }
                    }
                    
                    let minor = (cofactor[0][0] * cofactor[1][1] - cofactor[0][1] * cofactor[1][0]) % 26;
                    let cofactor_value = if (i + j) % 2 == 0 { minor } else { -minor };
                    inverse[j][i] = (cofactor_value * det_inv) % 26;
                }
            }
        }
        
        for i in 0..n {
            for j in 0..n {
                inverse[i][j] = (inverse[i][j] + 26) % 26;
            }
        }
        
        Ok(inverse)
    }
    
    pub fn encrypt(text: &str, key: &str) -> Result<String, String> {
        let matrix = Self::parse_key(key)?;
        let n = matrix.len();
        
        let text = text.to_uppercase();
        let mut result = String::new();
        
        let mut i = 0;
        while i < text.len() {
            let mut vector = Vec::with_capacity(n);
            
            for j in 0..n {
                if i + j < text.len() {
                    let c = text.chars().nth(i + j).unwrap();
                    if c.is_ascii_alphabetic() {
                        vector.push(char_to_index(c));
                    } else {
                        return Err("Text must contain only letters".to_string());
                    }
                } else {
                    vector.push(23);
                }
            }
            
            let encrypted = Self::matrix_multiply(&matrix, &vector);
            
            for &num in &encrypted {
                result.push(index_to_char(num, true));
            }
            
            i += n;
        }
        
        Ok(result)
    }
    
    pub fn decrypt(text: &str, key: &str) -> Result<String, String> {
        let matrix = Self::parse_key(key)?;
        let inverse = Self::matrix_inverse(&matrix)?;
        let n = matrix.len();
        
        let text = text.to_uppercase();
        let mut result = String::new();
        
        let mut i = 0;
        while i < text.len() {
            let mut vector = Vec::with_capacity(n);
            
            for j in 0..n {
                if i + j < text.len() {
                    let c = text.chars().nth(i + j).unwrap();
                    if c.is_ascii_alphabetic() {
                        vector.push(char_to_index(c));
                    } else {
                        return Err("Text must contain only letters".to_string());
                    }
                } else {
                    return Err("Invalid text length".to_string());
                }
            }
            
            let decrypted = Self::matrix_multiply(&inverse, &vector);
            
            for &num in &decrypted {
                result.push(index_to_char(num, true));
            }
            
            i += n;
        }
        
        Ok(result)
    }
} 