use super::{ SyntaxMetadata, SyntaxResult };
use crate::compiler::Token;

// Matches one token with given word
pub fn match_token(text: &String, expr: &[Token], meta: &mut SyntaxMetadata) -> Option<SyntaxResult> {
    match expr.get(meta.index) {
        Some(token) => if token.word == *text {
            meta.index += 1;
            Some(SyntaxResult::Word(token.word.clone()))
        } else { None }
        None => None
    }
}
