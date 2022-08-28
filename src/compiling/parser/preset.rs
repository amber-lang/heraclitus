use crate::compiling::logging::ErrorDetails;
use super::Metadata;

/// Match variable name
/// 
/// Matches one token with a word that would be considered as a variable name.
/// If desired - one can extend this implementation with other chars.
pub fn variable(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    match meta.get_current_token() {
        Some(token) => {
            // This boolean stores false if we are past
            // the first letter otherwise it's true
            let mut is_later: bool = false;
            for letter in token.word.chars() {
                // Check if rest of the letters are alphanumeric
                if is_later {
                    if !(letter.is_alphanumeric() || extend.contains(&letter)) {
                        return Err(ErrorDetails::from_token_option(Some(token)))
                    }
                }
                // Check if first letter is alphabetic
                else {
                    if !(letter.is_alphabetic() || extend.contains(&letter)) {
                        return Err(ErrorDetails::from_token_option(Some(token)))
                    }
                    is_later = true;
                }
            }
            meta.increment_index();
            Ok(token.word)
        }
        None => Err(ErrorDetails::with_eof())
    }
}

/// Match alphabetic word
///
/// Matches one token with a word that consists of letters only.
/// If desired - one can extend this implementation with other chars.
pub fn alphabetic(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    match meta.get_current_token() {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphabetic() || extend.contains(&letter)) {
                meta.increment_index();
                Ok(token.word)
            } else { Err(ErrorDetails::from_token_option(Some(token))) }
        }
        None => Err(ErrorDetails::with_eof())
    }
}

/// Match alphanumeric word
/// 
/// Matches one token with a word that consists of letters or numbers only.
/// If desired - one can extend this implementation with other chars.
pub fn alphanumeric(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    match meta.get_current_token() {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphanumeric() || extend.contains(&letter)) {
                meta.increment_index();
                Ok(token.word)
            } else { Err(ErrorDetails::from_token_option(Some(token))) }
        }
        None => Err(ErrorDetails::with_eof())
    }
}

/// Match numeric word
/// 
/// Matches a token of which word is a string of digits.
/// If desired - one can extend this implementation with other chars.
pub fn numeric(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    match meta.get_current_token() {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_numeric() || extend.contains(&letter)) {
                meta.increment_index();
                Ok(token.word)
            } else { Err(ErrorDetails::from_token_option(Some(token))) }
        }
        None => Err(ErrorDetails::with_eof())
    }
}

/// Match an integer
/// 
/// Matches a positive or negetive integer.
/// If desired - one can extend this implementation with other chars.
pub fn integer(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    match meta.get_current_token() {
        Some(token) => {
            let mut word = token.word.clone();
            // If it's a negative number - consume
            if word.starts_with('-') {
                word = word[1..].chars().collect();
            }
            // For each further letter match a digit
            for letter in word.chars() {
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return Err(ErrorDetails::from_token_option(Some(token)))
                }
            }
            meta.increment_index();
            Ok(token.word)
        }
        None => Err(ErrorDetails::with_eof())
    }
}

/// Match a float
/// 
/// Matches a number that contains a floating point (has a dot in decimal notation)
/// If desired - one can extend this implementation with other chars.
pub fn float(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    match meta.get_current_token() {
        Some(token) => {
            let mut word = token.word.clone();
            // If it's a negative number - consume
            if word.starts_with('-') {
                word = word[1..].chars().collect();
            }
            // Determine if 'dot' was found
            let mut is_frac = false;
            for letter in word.chars() {
                if letter == '.' {
                    // Set fraction if dot - exit match otherwise
                    is_frac = if is_frac { return Err(ErrorDetails::from_token_option(Some(token))) } else { true };
                    continue
                }
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return Err(ErrorDetails::from_token_option(Some(token)))
                }
            }
            meta.increment_index();
            Ok(token.word)
        }
        None => Err(ErrorDetails::with_eof())
    }
}

/// Match a number
/// 
/// Matches a number that is an integer or float
/// If desired - one can extend this implementation with other chars.
pub fn number(meta: &mut impl Metadata, extend: Vec<char>) -> Result<String, ErrorDetails> {
    if let Ok(integer) = integer(meta, extend.clone()) {
        return Ok(integer);
    }
    float(meta, extend)
}
