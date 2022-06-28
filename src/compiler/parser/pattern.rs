use super::{ SyntaxMetadata };

// Matches one token with given word
pub fn token<T: AsRef<str>>(meta: &mut SyntaxMetadata, text: T) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => if token.word == text.as_ref() {
            meta.index += 1;
            Ok(token.word.clone())
        } else { Err(()) }
        None => Err(())
    }
}