//! Simple error structure
//! 
//! This module defines `ErrorDetails` structure which is returned as
//! an error by lexer and is used in parsing phase as well.

use std::fs::File;
use std::io::*;
use crate::compiler::{Metadata, Token};

/// Store position of some error
#[derive(Debug)]
pub enum ErrorPosition {
    /// Explicit row and column
    Pos(usize, usize),
    /// End of file
    EOF
}

/// Struct that is used to return a simple error
#[derive(Debug)]
pub struct ErrorDetails {
    /// Location of this error
    pub position: ErrorPosition,
    /// Length of the token
    pub len: usize,
    /// Additional information
    pub data: Option<String>,
}

impl ErrorDetails {
    /// Create a new erorr from scratch
    pub fn new(position: ErrorPosition, len: usize) -> Self {
        ErrorDetails {
            position,
            len,
            data: None
        }
    }

    /// Create a new erorr at the end of file
    pub fn with_eof() -> Self {
        ErrorDetails {
            position: ErrorPosition::EOF,
            len: 0,
            data: None
        }
    }

    /// Create a new erorr at given position
    pub fn with_pos((row, col): (usize, usize), len: usize) -> Self {
        ErrorDetails {
            position: ErrorPosition::Pos(row, col),
            len,
            data: None
        }
    }

    /// Create an error at current position of current token by metadata
    /// 
    /// This function can become handy when parsing the AST.
    /// This takes the current index stored in metadata and uses it
    /// to retrieve token stored under it in metadata's expression.
    /// Then it's position is used to express the ErrorPosition
    pub fn from_metadata(meta: &impl Metadata) -> Self {
        Self::from_token_option(meta.get_current_token())
    }

    /// Create an error at position of the provided token
    /// 
    /// This function gives you ability to store tokens
    /// and error once you finished parsing the entire expression
    pub fn from_token_option(token_opt: Option<Token>) -> Self {
        match token_opt {
            Some(token) => ErrorDetails::with_pos(token.pos, token.word.len()),
            None => ErrorDetails::with_eof()
        }
    }

    /// Attach additional data in form of a string
    pub fn data<T: AsRef<str>>(mut self, data: T) -> Self {
        self.data = Some(data.as_ref().to_string().clone());
        self
    }

    /// Get position of the error by either path or code
    pub fn get_pos_by_file_or_code(&self, path: Option<String>, code: Option<String>) -> (usize, usize) {
        match self.position {
            ErrorPosition::Pos(row, col) => (row, col),
            ErrorPosition::EOF => {
                if let Some(code) = code {
                    self.get_pos_by_code(code)
                }
                else if let Some(path) = path {
                    match self.get_pos_by_file(path) {
                        Ok((row, col)) => (row, col),
                        Err(_) => (0, 0)
                    }
                }
                else { (0, 0) }
            }
        }
    }

    /// In case of EOF this function ensures you to return concrete position
    pub fn get_pos_by_file(&self, path: impl AsRef<str>) -> std::io::Result<(usize, usize)> {
        let mut code = format!("");
        let mut file = File::open(path.as_ref())?;
        file.read_to_string(&mut code)?;
        Ok(self.get_pos_by_code(&code))
    }

    /// In case of EOF this function ensures you to return concrete position
    pub fn get_pos_by_code(&self, code: impl AsRef<str>) -> (usize, usize) {
        let code = code.as_ref();
        match self.position {
            ErrorPosition::Pos(row, col) => (row, col),
            ErrorPosition::EOF => {
                let mut col = 1;
                let mut row = 1;
                // Count letters in column
                col += code.split_whitespace().count();
                // Coint letters in row
                if let Some(last) = code.split_whitespace().last() {
                    row += last.len();
                }
                return (row, col);
            }
        }
    }
}
