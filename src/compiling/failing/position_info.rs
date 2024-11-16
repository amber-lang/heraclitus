//! Simple error structure
//!
//! This module defines `ErrorDetails` structure which is returned as
//! an error by lexer and is used in parsing phase as well.

use std::fs::File;
use std::io::*;
use crate::compiling::{Metadata, Token};

/// Store position of some error
#[derive(Debug, Clone)]
pub enum Position {
    /// Explicit row and column
    Pos(usize, usize),
    /// End of file
    EOF
}

/// Struct that is used to return a simple error
#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// Path of the file
    pub path: Option<String>,
    /// Location of this error
    pub position: Position,
    /// Length of the token
    pub len: usize,
    /// Additional information
    pub data: Option<String>
}

impl PositionInfo {
    /// Create a new erorr from scratch
    pub fn new(meta: &impl Metadata, position: Position, len: usize) -> Self {
        let info = PositionInfo {
            position,
            path: meta.get_path(),
            len,
            data: None
        };
        info.updated_pos(meta)
    }

    /// Create a new erorr at the end of file
    pub fn at_eof(meta: &impl Metadata) -> Self {
        let info = PositionInfo {
            path: meta.get_path(),
            position: Position::EOF,
            len: 0,
            data: None
        };
        info.updated_pos(meta)
    }

    /// Create a new erorr at given position
    pub fn at_pos(path: Option<String>, (row, col): (usize, usize), len: usize) -> Self {
        PositionInfo {
            path,
            position: Position::Pos(row, col),
            len,
            data: None
        }
    }

    fn updated_pos(mut self, meta: &impl Metadata) -> Self {
        let (row, col) = self.get_pos_by_file_or_code(meta.get_code());
        self.position = Position::Pos(row, col);
        self
    }

    /// Get the path to the file and return [unknown] if not supplied
    pub fn get_path(&self) -> String {
        self.path.clone().unwrap_or_else(|| "[unknown]".to_string())
    }

    /// Create an error at current position of current token by metadata
    ///
    /// This function can become handy when parsing the AST.
    /// This takes the current index stored in metadata and uses it
    /// to retrieve token stored under it in metadata's expression.
    /// Then it's position is used to express the ErrorPosition
    pub fn from_metadata(meta: &impl Metadata) -> Self {
        Self::from_token(meta, meta.get_current_token())
    }

    /// Create an error at position of the provided token
    ///
    /// This function gives you ability to store tokens
    /// and error once you finished parsing the entire expression
    pub fn from_token(meta: &impl Metadata, token_opt: Option<Token>) -> Self {
        match token_opt {
            Some(token) => PositionInfo::at_pos(meta.get_path(), token.pos, token.word.chars().count()),
            None => PositionInfo::at_eof(meta)
        }
    }

    /// Create an error at position between two tokens
    ///
    /// This function is used to create an error between two tokens
    /// which can be used to express an error in a specific range
    pub fn from_between_tokens(meta: &impl Metadata, begin: Option<Token>, end: Option<Token>) -> Self {
        if let Some(begin) = begin {
            let (row, col) = begin.pos;
            let end = end.map_or(usize::max_value(), |tok| tok.start);
            let len = end - begin.start;
            PositionInfo::at_pos(meta.get_path(), (row, col), len)
        }
        else {
            PositionInfo::from_metadata(meta)
        }
    }

    /// Attach additional data in form of a string
    pub fn data<T: AsRef<str>>(mut self, data: T) -> Self {
        self.data = Some(data.as_ref().to_string());
        self
    }

    /// Get position of the error by either path or code
    pub fn get_pos_by_file_or_code(&self, code: Option<&String>) -> (usize, usize) {
        match self.position {
            Position::Pos(row, col) => (row, col),
            Position::EOF => {
                if let Some(code) = code {
                    self.get_pos_by_code(code)
                }
                else if let Some(path) = &self.path {
                    match self.get_pos_by_file(path) {
                        Ok((row, col)) => (row, col),
                        Err(_) => (0, 0)
                    }
                }
                else {
                    (0, 0)
                }
            }
        }
    }

    /// In case of EOF this function ensures you to return concrete position
    pub fn get_pos_by_file(&self, path: impl AsRef<str>) -> std::io::Result<(usize, usize)> {
        let mut code = String::new();
        let mut file = File::open(path.as_ref())?;
        file.read_to_string(&mut code)?;
        Ok(self.get_pos_by_code(&code))
    }

    /// In case of EOF this function ensures you to return concrete position
    pub fn get_pos_by_code(&self, code: impl AsRef<str>) -> (usize, usize) {
        let code = code.as_ref();
        match self.position {
            Position::Pos(row, col) => {
                (row, col)
            }
            Position::EOF => {
                // Add one to `row` because `enumerate()` counts from zero.
                // Add one to `col` because `len()` counts from zero.
                if let Some((row, line)) = code.lines().enumerate().last() {
                    (row + 1, line.len() + 1)
                } else {
                    (0, 0)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::DefaultMetadata;
    use super::*;

    #[test]
    fn test_position_info() {
        let pos = PositionInfo::at_pos(Some("test".to_string()), (1, 1), 1);
        assert_eq!(pos.get_path(), "test");
        assert_eq!(pos.get_pos_by_code("test"), (1, 1));
    }

    #[test]
    fn test_position_info_between_tokens() {
        let begin = Token { word: "begin".to_string(), pos: (1, 1), start: 0 };
        let to = Token { word: "to".to_string(), pos: (1, 7), start: 6 };
        let end = Token { word: "end".to_string(), pos: (1, 10), start: 9 };
        let mut meta = DefaultMetadata::new(vec![begin.clone(), to.clone(), end.clone()], None, Some("begin to end".to_string()));
        let pos = PositionInfo::from_between_tokens(&mut meta, Some(begin.clone()), Some(end.clone()));
        assert_eq!(pos.len, end.start - begin.start);
    }
}
