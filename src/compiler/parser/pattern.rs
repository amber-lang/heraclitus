use crate::compiler::Token;

// Matches one token with given word
pub fn match_token(text: &&str, expr: &[Token], index: &mut usize) -> bool {
    match expr.get(*index) {
        Some(token) => if token.word == String::from(*text) {
            *index += 1;
            true
        } else {
            false
        }
        None => false
    }
}
