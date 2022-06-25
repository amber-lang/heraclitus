use std::vec;

use super::pattern::match_token;
use crate::compiler::Token;

// Matches one token with a word that would be considered as a variable name
pub fn match_variable(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    match expr.get(*index) {
        Some(token) => {
            // This boolean stores false if we are past
            // the first letter otherwise it's true
            let mut is_later: bool = false;
            for letter in token.word.chars() {
                // Check if rest of the letters are alphanumeric
                if is_later {
                    if !(letter.is_alphanumeric() || extend.contains(&letter)) {
                        return false
                    }
                }
                // Check if first letter is alphabetic
                else {
                    if !(letter.is_alphabetic() || extend.contains(&letter)) {
                        return false
                    }
                    is_later = true;
                }
            }
            *index += 1;
            true
        }
        None => false
    }
}

// Matches one token with a word that consists of letters only
pub fn match_alphabetic(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    match expr.get(*index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphabetic() || extend.contains(&letter)) {
                *index += 1;
                true
            } else { false }
        }
        None => false
    }
}

// Matches one token with a word that consists of letters or numbers only
pub fn match_alphanumeric(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    match expr.get(*index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphanumeric() || extend.contains(&letter)) {
                *index += 1;
                true
            } else { false }
        }
        None => false
    }
}

// Matches a token of which word is a string of digits
pub fn match_numeric(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    match expr.get(*index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_numeric() || extend.contains(&letter)) {
                *index += 1;
                true
            } else { false }
        },
        None => false
    }
}

// Matches a positive or negetive integer
pub fn match_integer(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    match expr.get(*index) {
        Some(token) => {
            let mut word = token.word.clone();
            // If it's a negative number - consume
            if word.chars().next() == Some('-') {
                word = word[1..].chars().collect();
            }
            // For each further letter match a digit
            for letter in word.chars() {
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return false
                }
            }
            *index += 1;
            true
        }
        None => false
    }
}

// Matches a number that contains a floating point (has a dot in notation)
pub fn match_float(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    match expr.get(*index) {
        Some(token) => {
            let mut word = token.word.clone();
            // If it's a negative number - consume
            if word.chars().next() == Some('-') {
                word = word[1..].chars().collect();
            }
            // Determine if 'dot' was found
            let mut is_frac = false;
            for letter in word.chars() {
                if letter == '.' {
                    // Set fraction if dot - exit match otherwise
                    is_frac = if is_frac { return false } else { true };
                    continue
                }
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    println!("FAIL");
                    return false
                }
            }
            *index += 1;
            true
        }
        None => false
    }
}

// Matches a number that is an integer or float
pub fn match_number(expr: &[Token], index: &mut usize, extend: &Vec<char>) -> bool {
    let is_int = match_integer(expr, index, extend);
    if is_int { is_int }
    else { match_float(expr, index, extend) }
}