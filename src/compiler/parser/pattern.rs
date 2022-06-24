use crate::compiler::Token;

// Matches one token with given word
pub fn match_token(text: &String, expr: &[Token], index: &mut usize) -> bool {
    match expr.get(*index) {
        Some(token) => if token.word == *text {
            *index += 1;
            true
        } else {
            false
        }
        None => false
    }
}
