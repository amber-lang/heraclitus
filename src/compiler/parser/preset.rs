use super::{ SyntaxMetadata };

// Matches one token with a word that would be considered as a variable name
pub fn variable(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => {
            // This boolean stores false if we are past
            // the first letter otherwise it's true
            let mut is_later: bool = false;
            for letter in token.word.chars() {
                // Check if rest of the letters are alphanumeric
                if is_later {
                    if !(letter.is_alphanumeric() || extend.contains(&letter)) {
                        return Err(())
                    }
                }
                // Check if first letter is alphabetic
                else {
                    if !(letter.is_alphabetic() || extend.contains(&letter)) {
                        return Err(())
                    }
                    is_later = true;
                }
            }
            meta.index += 1;
            Ok(token.word.clone())
        }
        None => Err(())
    }
}

// Matches one token with a word that consists of letters only
pub fn alphabetic(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphabetic() || extend.contains(&letter)) {
                meta.index += 1;
                Ok(token.word.clone())
            } else { Err(()) }
        }
        None => Err(())
    }
}

// Matches one token with a word that consists of letters or numbers only
pub fn match_alphanumeric(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_alphanumeric() || extend.contains(&letter)) {
                meta.index += 1;
                Ok(token.word.clone())
            } else { Err(()) }
        }
        None => Err(())
    }
}

// Matches a token of which word is a string of digits
pub fn numeric(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_numeric() || extend.contains(&letter)) {
                meta.index += 1;
                Ok(token.word.clone())
            } else { Err(()) }
        }
        None => Err(())
    }
}

// Matches a positive or negetive integer
pub fn integer(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => {
            let mut word = token.word.clone();
            // If it's a negative number - consume
            if word.chars().next() == Some('-') {
                word = word[1..].chars().collect();
            }
            // For each further letter match a digit
            for letter in word.chars() {
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return Err(())
                }
            }
            meta.index += 1;
            Ok(token.word.clone())
        }
        None => Err(())
    }
}

// Matches a number that contains a floating point (has a dot in notation)
pub fn float(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    match meta.expr.get(meta.index) {
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
                    is_frac = if is_frac { return Err(()) } else { true };
                    continue
                }
                if !(letter.is_numeric() || extend.contains(&letter)) {
                    return Err(())
                }
            }
            meta.index += 1;
            Ok(token.word.clone())
        }
        None => Err(())
    }
}

// Matches a number that is an integer or float
pub fn number(meta: &mut SyntaxMetadata, extend: Vec<char>) -> Result<String,()> {
    if let Ok(integer) = integer(meta, extend.clone()) {
        return Ok(integer);
    }
    float(meta, extend)
}