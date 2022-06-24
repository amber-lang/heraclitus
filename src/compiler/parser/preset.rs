use super::pattern::match_token;
use crate::compiler::Token;

// Matches one token with a word that would be considered as a variable name
pub fn match_variable(expr: &[Token], index: &mut usize) -> bool {
    match expr.get(*index) {
        Some(token) => {
            // This boolean stores false if we are past
            // the first letter otherwise it's true
            let mut is_later: bool = false;
            for letter in token.word.chars() {
                // Check if rest of the letters are alphanumeric
                if is_later {
                    if !(letter.is_alphanumeric() || letter == '_') {
                        return false;
                    }
                }
                // Check if first letter is alphabetic
                else {
                    if !(letter.is_alphabetic() || letter == '_') {
                        return false;
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

// Matches a token of which word is a string of digits
pub fn match_numeric(expr: &[Token], index: &mut usize) -> bool {
    match expr.get(*index) {
        Some(token) => {
            if token.word.chars().all(|letter| letter.is_numeric()) {
                *index += 1;
                true
            } else { false }
        },
        None => false
    }
}

// Matches a positive or negetive integer
pub fn match_integer(expr: &[Token], index: &mut usize) -> bool {
    // Save index in case if this function fails
    let mut new_index = index.clone();
    match_token(&"-", expr, &mut new_index);
    *index = new_index;
    match_numeric(expr, index)
}

// Matches a number that contains a floating point (has a dot in notation)
pub fn match_float(expr: &[Token], index: &mut usize) -> bool {
    // Save index in case if this function fails
    let mut new_index = index.clone();
    if match_integer(expr, &mut new_index) {
        match_token(&".", expr, &mut new_index);
        *index = new_index;
        match_numeric(expr, index)
    } else {
        match_token(&"-", expr, &mut new_index);
        match_token(&".", expr, &mut new_index);
        *index = new_index;
        match_numeric(expr, index)
    }
}

// Matches a number that is an integer or float
pub fn match_number(expr: &[Token], index: &mut usize) -> bool {
    let is_int = match_integer(expr, index);
    let is_float = match_float(expr, index);
    println!("{is_int}, {is_float}");
    is_int || is_float
}
