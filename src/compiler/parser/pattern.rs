use super::{ Metadata, SyntaxModule };
use super::util::meta_index_increment;

// Matches one token with given word
pub fn token<T: AsRef<str>>(meta: &mut impl Metadata, text: T) -> Result<String,()> {
    match meta.get_token_at(meta.get_index()) {
        Some(token) => if token.word == text.as_ref() {
            meta_index_increment(meta);
            Ok(token.word.clone())
        } else { Err(()) }
        None => Err(())
    }
}

// Matches one token with given word
pub fn token_by(meta: &mut impl Metadata, cb: impl Fn(&String) -> bool) -> Result<String,()> {
    match meta.get_token_at(meta.get_index()) {
        Some(token) => if cb(&token.word) {
            meta_index_increment(meta);
            Ok(token.word.clone())
        } else { Err(()) }
        None => Err(())
    }
}

// Parses syntax and returns it's result
pub fn syntax<M: Metadata>(meta: &mut M, module: &mut impl SyntaxModule<M>) -> Result<(),()> {
    let index = meta.get_index();
    if let Err(()) = module.parse(meta) {
        meta.set_index(index);
        Err(())
    } else { Ok(()) }
}

// Parses indentation
pub fn indent(meta: &mut impl Metadata) -> Result<usize, ()> {
    let fun = |word: &String| word.starts_with('\n') && word.get(1..).unwrap().chars().all(|letter| letter == ' ');
    if let Ok(word) = token_by(meta, fun) {
        Ok(word.get(1..).unwrap().len())
    } else { Err(()) }
}

// Parses indentation
pub fn indent_with(meta: &mut impl Metadata, size: usize) -> Result<std::cmp::Ordering, ()> {
    let index = meta.get_index();
    let fun = |word: &String| word.starts_with('\n') && word.get(1..).unwrap().chars().all(|letter| letter == ' ');
    if let Ok(word) = token_by(meta, fun) {
        let spaces = word.len() - 1;
        Ok(spaces.cmp(&size))
    }
    else {
        meta.set_index(index);
        Err(())
    }
}

#[cfg(test)]
mod test {
    use crate::{SyntaxMetadata, Token};
    use super::*;

    #[test]
    fn indent_test() {
        let expr = vec![Token {word: format!("\n    "), pos: (0, 0)}];
        let mut meta = SyntaxMetadata::new(expr);
        let res = indent(&mut meta);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn indent_with_test() {
        let expr = vec![Token { word: format!("\n    "), pos: (0, 0) }];
        let mut meta = SyntaxMetadata::new(expr);
        let res = indent_with(&mut meta, 4);
        assert!(res.is_ok());
    }
}