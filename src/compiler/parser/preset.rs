use super::{SyntaxMetadata, SyntaxResult};
use crate::compiler::Token;

// Matches one token with a word that would be considered as a variable name
pub fn match_variable(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
        Some(token) => {
            // This boolean stores false if we are past
            // the first letter otherwise it's true
            let mut is_later: bool = false;
            for letter in token.word.chars() {
                // Check if rest of the letters are alphanumeric
                if is_later {
                    if !(letter.is_alphanumeric() || extend.contains(&letter)) {
                        return None
                    }
                }
                // Check if first letter is alphabetic
                else {
                    if !(letter.is_alphabetic() || extend.contains(&letter)) {
                        return None
                    }
                    is_later = true;
                }
            }
            meta.index += 1;
            Some(SyntaxResult::Word(token.word.clone()))
        }
        None => None
    }
}

// Matches one token with a word that consists of letters only
pub fn match_alphabetic(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphabetic() || extend.contains(&letter)) {
                meta.index += 1;
                Some(SyntaxResult::Word(token.word.clone()))
            } else { None }
        }
        None => None
    }
}

// Matches one token with a word that consists of letters or numbers only
pub fn match_alphanumeric(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphanumeric() || extend.contains(&letter)) {
                meta.index += 1;
                Some(SyntaxResult::Word(token.word.clone()))
            } else { None }
        }
        None => None
    }
}

// Matches a token of which word is a string of digits
pub fn match_numeric(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_numeric() || extend.contains(&letter)) {
                meta.index += 1;
                Some(SyntaxResult::Word(token.word.clone()))
            } else { None }
        }
        None => None
    }
}

// Matches a positive or negetive integer
pub fn match_integer(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
        Some(token) => {
            let mut word = token.word.clone();
            // If it's a negative number - consume
            if word.chars().next() == Some('-') {
                word = word[1..].chars().collect();
            }
            // For each further letter match a digit
            for letter in word.chars() {
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return None
                }
            }
            meta.index += 1;
            Some(SyntaxResult::Word(token.word.clone()))
        }
        None => None
    }
}

// Matches a number that contains a floating point (has a dot in notation)
pub fn match_float(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
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
                    is_frac = if is_frac { return None } else { true };
                    continue
                }
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return None
                }
            }
            meta.index += 1;
            Some(SyntaxResult::Word(token.word.clone()))
        }
        None => None
    }
}

// Matches a number that is an integer or float
pub fn match_number(expr: &[Token], meta: &mut SyntaxMetadata, extend: &Vec<char>) -> Option<SyntaxResult> {
    if let Some(integer) = match_integer(expr, meta, extend) {
        return Some(integer);
    }
    return match_float(expr, meta, extend)
}