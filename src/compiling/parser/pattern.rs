use crate::compiling::failing::position_info::PositionInfo;
use crate::compiling::failing::failure::Failure;
use super::{ Metadata, SyntaxModule };

/// Matches one token with given word
///
/// If token was matched succesfully - the word it contained is returned.
/// Otherwise detailed information is returned about where this happened.
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn compile() -> Result<(), Failure> {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// token(meta, "let")?;
/// # Ok(())
/// # }
/// ```
pub fn token<T: AsRef<str>>(meta: &mut impl Metadata, text: T) -> Result<String, Failure> {
    match meta.get_current_token() {
        Some(token) => if token.word == text.as_ref() {
            meta.increment_index();
            Ok(token.word)
        } else { Err(Failure::Quiet(PositionInfo::from_token(meta, Some(token)))) }
        None => Err(Failure::Quiet(PositionInfo::at_eof(meta)))
    }
}

/// Matches one token by defined function
///
/// If token was matched succesfully - the word it contained is returned.
/// Otherwise detailed information is returned about where this happened.
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn compile() -> Result<(), Failure> {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// let the_word = token_by(meta, |word| word.starts_with('@'))?;
/// # Ok(())
/// # }
/// ```
pub fn token_by(meta: &mut impl Metadata, cb: impl Fn(&String) -> bool) -> Result<String, Failure> {
    match meta.get_current_token() {
        Some(token) => if cb(&token.word) {
            meta.increment_index();
            Ok(token.word)
        } else { Err(Failure::Quiet(PositionInfo::from_token(meta, Some(token)))) }
        None => Err(Failure::Quiet(PositionInfo::at_eof(meta)))
    }
}

/// Parses syntax module
///
/// If syntax module was parsed succesfully - nothing is returned.
/// Otherwise detailed information is returned about where this happened.
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # struct IfStatement {}
/// # impl SyntaxModule<DefaultMetadata> for IfStatement {
/// #   syntax_name!("If");
/// #   fn new() -> Self { IfStatement {} }
/// #   fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult { Ok(()) }
/// # }
/// # fn compile() -> Result<(), Failure> {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// let mut ifst = IfStatement::new();
/// syntax(meta, &mut ifst)?;
/// # Ok(())
/// # }
/// ```
pub fn syntax<M: Metadata>(meta: &mut M, module: &mut impl SyntaxModule<M>) -> Result<(), Failure> {
    let index = meta.get_index();
    // Determine if we shall parse it in debug mode or not
    let result = match meta.get_debug() {
        Some(_) => module.parse_debug(meta),
        None => module.parse(meta)
    };
    if let Err(failure) = result {
        meta.set_index(index);
        Err(failure)
    } else { Ok(()) }
}

/// Matches indentation
///
/// If indentation was matched succesfully - the amount of spaces is returned.
/// Otherwise detailed information is returned about where this happened.
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn compile() -> Result<(), Failure> {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// let spaces: usize = indent(meta)?;
/// # Ok(())
/// # }
/// ```
pub fn indent(meta: &mut impl Metadata) -> Result<usize, Failure> {
    let fun = |word: &String| word.starts_with('\n') && word.get(1..).unwrap().chars().all(|letter| letter == ' ');
    match token_by(meta, fun) {
        Ok(word) => Ok(word.get(1..).unwrap().len()),
        Err(failure) => Err(failure)
    }
}

/// Matches indentation with provided size
///
/// If indentation was identified succesfully return the std::cmp::Ordering
/// depending on whether the amount of spaces detected was smaller, equal or greater.
/// Otherwise detailed information is returned about where this happened.
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn compile() -> Result<(), Failure> {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// let cmp: std::cmp::Ordering = indent_with(meta, 6)?;
/// # Ok(())
/// # }
/// ```
pub fn indent_with(meta: &mut impl Metadata, size: usize) -> Result<std::cmp::Ordering, Failure> {
    let index = meta.get_index();
    let fun = |word: &String| word.starts_with('\n') && word.get(1..).unwrap().chars().all(|letter| letter == ' ');
    match token_by(meta, fun) {
        Ok(word) => {
            let spaces = word.chars().count() - 1;
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
    use crate::compiling::{DefaultMetadata, Token};
    use super::*;

    #[test]
    fn indent_test() {
        let expr = vec![Token {word: format!("\n    "), pos: (0, 0), start: 0 }];
        let mut meta = DefaultMetadata::new(expr, Some(format!("path/to/file")), None);
        let res = indent(&mut meta);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn indent_with_test() {
        let expr = vec![Token { word: format!("\n    "), pos: (0, 0), start: 0 }];
        let mut meta = DefaultMetadata::new(expr, Some(format!("path/to/file")), None);
        let res = indent_with(&mut meta, 4);
        assert!(res.is_ok());
    }
}
