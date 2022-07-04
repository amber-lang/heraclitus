use super::{ SyntaxMetadata, SyntaxModule };

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

// Matches one token with given word
pub fn token_by(meta: &mut SyntaxMetadata, cb: fn(&String) -> bool) -> Result<String,()> {
    match meta.expr.get(meta.index) {
        Some(token) => if cb(&token.word) {
            meta.index += 1;
            Ok(token.word.clone())
        } else { Err(()) }
        None => Err(())
    }
}

// Parses syntax and returns it's result
pub fn syntax(meta: &mut SyntaxMetadata, module: &mut impl SyntaxModule) -> Result<(),()> {
    let index = meta.index;
    if let Err(()) = module.parse(meta) {
        meta.index = index;
        Err(())
    } else { Ok(()) }
}
