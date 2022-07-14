use std::fs::File;
use std::io::*;
use crate::Metadata;

#[derive(Debug)]
pub enum ErrorLocation {
    Pos(usize, usize),
    EOF
}

#[derive(Debug)]
pub struct ErrorDetails {
    pub location: ErrorLocation,
    pub data: Option<String>,
}

impl ErrorDetails {
    pub fn new(location: ErrorLocation) -> Self {
        ErrorDetails {
            location,
            data: None
        }
    }

    pub fn with_eof() -> Self {
        ErrorDetails {
            location: ErrorLocation::EOF,
            data: None
        }
    }

    pub fn with_pos((row, col): (usize, usize)) -> Self {
        ErrorDetails {
            location: ErrorLocation::Pos(row, col),
            data: None
        }
    }

    pub fn from_metadata(meta: &impl Metadata) -> Self {
        match meta.get_token_at(meta.get_index()) {
            Some(token) => ErrorDetails::with_pos(token.pos),
            None => ErrorDetails::with_eof()
        }
    }

    pub fn data<T: AsRef<str>>(mut self, data: T) -> Self {
        self.data = Some(data.as_ref().to_string().clone());
        self
    }

    pub fn get_pos_by_file(&mut self, path: &String) -> std::io::Result<(usize, usize)> {
        let mut code = format!("");
        let mut file = File::open(path)?;
        file.read_to_string(&mut code)?;
        Ok(self.get_pos_by_code(&code))
    }

    /// Returns a position 
    pub fn get_pos_by_code(&mut self, code: &String) -> (usize, usize) {
        match self.location {
            ErrorLocation::Pos(row, col) => (row, col),
            ErrorLocation::EOF => {
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
