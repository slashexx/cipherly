use std::collections::HashMap;

pub fn char_to_index(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        (c as i32) - ('A' as i32)
    } else if c.is_ascii_lowercase() {
        (c as i32) - ('a' as i32)
    } else {
        -1
    }
}

pub fn index_to_char(index: i32, uppercase: bool) -> char {
    let base = if uppercase { 'A' } else { 'a' };
    ((base as i32) + (index.rem_euclid(26))) as u8 as char
}

pub fn calculate_frequency(text: &str) -> HashMap<char, f64> {
    let mut freq = HashMap::new();
    let mut total = 0;no 
    
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            *freq.entry(c.to_ascii_uppercase()).or_insert(0.0) += 1.0;
            total += 1;
        }
    }
    
    if total > 0 {
        for (_, count) in freq.iter_mut() {
            *count /= total as f64;
        }
    }
    
    freq
}

pub const ENGLISH_FREQUENCIES: [(char, f64); 26] = [
    ('E', 0.12702), ('T', 0.09056), ('A', 0.08167), ('O', 0.07507),
    ('I', 0.06966), ('N', 0.06749), ('S', 0.06327), ('H', 0.06094),
    ('R', 0.05987), ('D', 0.04253), ('L', 0.04025), ('C', 0.02782),
    ('U', 0.02758), ('M', 0.02406), ('W', 0.02360), ('F', 0.02228),
    ('G', 0.02015), ('Y', 0.01974), ('P', 0.01929), ('B', 0.01492),
    ('V', 0.00978), ('K', 0.00772), ('J', 0.00153), ('X', 0.00150),
    ('Q', 0.00095), ('Z', 0.00074),
];

pub fn chi_squared(freq: &HashMap<char, f64>) -> f64 {
    let mut chi_squared = 0.0;
    
    for (letter, expected_freq) in ENGLISH_FREQUENCIES.iter() {
        let observed_freq = freq.get(letter).unwrap_or(&0.0);
        let diff = observed_freq - expected_freq;
        chi_squared += (diff * diff) / expected_freq;
    }
    
    chi_squared
}