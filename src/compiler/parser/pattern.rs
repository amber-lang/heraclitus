use crate::compiler::logger::ErrorDetails;
use super::{ Metadata, SyntaxModule };

// Matches one token with given word
pub fn token<T: AsRef<str>>(meta: &mut impl Metadata, text: T) -> Result<String, ErrorDetails> {
    match meta.get_token_at(meta.get_index()) {
        Some(token) => if token.word == text.as_ref() {
            meta.increment_index();
            Ok(token.word.clone())
        } else { Err(ErrorDetails::with_pos(token.pos)) }
        None => Err(ErrorDetails::with_eof())
    }
}

// Matches one token with given word
pub fn token_by(meta: &mut impl Metadata, cb: impl Fn(&String) -> bool) -> Result<String, ErrorDetails> {
    match meta.get_token_at(meta.get_index()) {
        Some(token) => if cb(&token.word) {
            meta.increment_index();
            Ok(token.word.clone())
        } else { Err(ErrorDetails::with_pos(token.pos)) }
        None => Err(ErrorDetails::with_eof())
    }
}

// Parses syntax and returns it's result
pub fn syntax<M: Metadata>(meta: &mut M, module: &mut impl SyntaxModule<M>) -> Result<(), ErrorDetails> {
    let index = meta.get_index();
    if let Err(details) = module.parse(meta) {
        meta.set_index(index);
        Err(details)
    } else { Ok(()) }
}

// Parses indentation
pub fn indent(meta: &mut impl Metadata) -> Result<usize, ErrorDetails> {
    let fun = |word: &String| word.starts_with('\n') && word.get(1..).unwrap().chars().all(|letter| letter == ' ');
    match token_by(meta, fun) {
        Ok(word) => Ok(word.get(1..).unwrap().len()),
        Err(details) => Err(details)
    }
}

// Parses indentation
pub fn indent_with(meta: &mut impl Metadata, size: usize) -> Result<std::cmp::Ordering, ErrorDetails> {
    let index = meta.get_index();
    let fun = |word: &String| word.starts_with('\n') && word.get(1..).unwrap().chars().all(|letter| letter == ' ');
    match token_by(meta, fun) {
        Ok(word) => {
            let spaces = word.len() - 1;
            Ok(spaces.cmp(&size))
        }
        Err(details) => {
            meta.set_index(index);
            Err(details)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{SyntaxMetadata, Token};
    use super::*;

    #[test]
    fn indent_test() {
        let expr = vec![Token {word: format!("\n    "), pos: (0, 0)}];
        let mut meta = SyntaxMetadata::new(expr, Some(format!("path/to/file")));
        let res = indent(&mut meta);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn indent_with_test() {
        let expr = vec![Token { word: format!("\n    "), pos: (0, 0) }];
        let mut meta = SyntaxMetadata::new(expr, Some(format!("path/to/file")));
        let res = indent_with(&mut meta, 4);
        assert!(res.is_ok());
    }
}