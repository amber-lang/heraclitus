//! Simple error structure
//! 
//! This module defines `ErrorDetails` structure which is returned as
//! an error by lexer and is used in parsing phase as well.

use std::fs::File;
use std::io::*;
use crate::compiler::Metadata;

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
    /// Additional information
    pub data: Option<String>,
}

impl ErrorDetails {
    /// Create a new erorr from scratch
    pub fn new(position: ErrorPosition) -> Self {
        ErrorDetails {
            position,
            data: None
        }
    }

    /// Create a new erorr at the end of file
    pub fn with_eof() -> Self {
        ErrorDetails {
            position: ErrorPosition::EOF,
            data: None
        }
    }

    /// Create a new erorr at given position
    pub fn with_pos((row, col): (usize, usize)) -> Self {
        ErrorDetails {
            position: ErrorPosition::Pos(row, col),
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
        match meta.get_token_at(meta.get_index()) {
            Some(token) => ErrorDetails::with_pos(token.pos),
            None => ErrorDetails::with_eof()
        }
    }

    /// Attach additional data in form of a string
    pub fn data<T: AsRef<str>>(mut self, data: T) -> Self {
        self.data = Some(data.as_ref().to_string().clone());
        self
    }

    /// In case of EOF this function ensures you to return concrete position
    pub fn get_pos_by_file(&mut self, path: &String) -> std::io::Result<(usize, usize)> {
        let mut code = format!("");
        let mut file = File::open(path)?;
        file.read_to_string(&mut code)?;
        Ok(self.get_pos_by_code(&code))
    }

    /// In case of EOF this function ensures you to return concrete position
    pub fn get_pos_by_code(&mut self, code: &String) -> (usize, usize) {
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
