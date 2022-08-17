//! Display your errors
//! 
//! Logger makes it easy for you to display errors. This sub-module is pretty powerful
//! to be used extensively instead of building own implementation of such mechanism.
//! However, if you need more specific functionality - it is encouraged to create your
//! own implementation of such mechanism.

#![allow(dead_code)]
use std::process;
use super::displayer::Displayer;
use crate::compiler::{Token};

/// Type of the message that logger shall display
#[derive(Clone)]
pub enum LogType {
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Info message
    Info
}

/// Logger itself
/// 
/// Log the message you want to show to the user
/// # Example
/// ```should_panic
/// # use heraclitus_compiler::prelude::*;
/// # let path = Some(format!("path/to/file"));
/// # let position = (0, 0);
/// # let guess = "type";
/// # let code = Some(format!("code"));
/// Logger::new_err_at_position(path, code, position)
///     .attach_message("Type of this parameter is invalid")
///     .attach_comment(format!("Maybe you meant type {guess} instead"))
///     .show()
///     .exit();
/// ```
pub struct Logger {
    /// Type of the message
    pub kind: LogType,
    /// Row position
    pub row: usize,
    /// Column position
    pub col: usize,
    /// Length of the token
    pub len: usize,
    /// Path to the source file
    pub path: Option<String>,
    /// Optionally store source code
    pub code: Option<String>,
    /// Optionally store message
    pub message: Option<String>,
    /// Optionally store comment
    pub comment: Option<String>
}

impl Logger {
    /// Create a new logger instance
    pub fn new(path: Option<String>, code: Option<String>, row: usize, col: usize, len: usize, kind: LogType) -> Self {
        Logger {
            kind,
            path,
            row,
            col,
            len,
            code,
            message: None,
            comment: None
        }
    }

    /// Create a new logger instance with message (suited for messages not related with issues in code)
    pub fn new_msg(message: impl AsRef<str>, kind: LogType) -> Self {
        Logger {
            kind,
            row: 0,
            col: 0,
            len: 0,
            path: None,
            code: None,
            message: Some(message.as_ref().to_string()),
            comment: None
        }
    }

    /// Show error message that does not relate to code
    pub fn new_err_msg(message: impl AsRef<str>) -> Self {
        Logger::new_msg(message, LogType::Error)
    }

    /// Show warning message that does not relate to code
    pub fn new_warn_msg(message: impl AsRef<str>) -> Self {
        Logger::new_msg(message, LogType::Warning)
    }

    /// Show info message that does not relate to code
    pub fn new_info_msg(message: impl AsRef<str>) -> Self {
        Logger::new_msg(message, LogType::Info)
    }

    /// Show error message based on the token
    pub fn new_err_with_token(path: Option<String>, code: Option<String>, token: Token) -> Self {
        Logger::new(path, code, token.pos.0, token.pos.1, token.word.len(), LogType::Error)
    }

    /// Show warning message based on the token
    pub fn new_warn_with_token(path: Option<String>, code: Option<String>, token: Token) -> Self {
        Logger::new(path, code, token.pos.0, token.pos.1, token.word.len(), LogType::Warning)
    }

    /// Show info message based on the token
    pub fn new_info_with_token(path: Option<String>, code: Option<String>, token: Token) -> Self {
        Logger::new(path, code, token.pos.0, token.pos.1, token.word.len(), LogType::Info)
    }

    /// Create an error by supplying essential information about the location
    pub fn new_err_at_position(path: Option<String>, code: Option<String>, (row, col): (usize, usize)) -> Self {
        Logger::new(path, code, row, col, 0, LogType::Error)
    }

    /// Create a warning by supplying essential information about the location
    pub fn new_warn_at_position(path: Option<String>, code: Option<String>, (row, col): (usize, usize)) -> Self {
        Logger::new(path, code, row, col, 0, LogType::Warning)
    }

    /// Create an info by supplying essential information about the location
    pub fn new_info_at_position(path: Option<String>, code: Option<String>, (row, col): (usize, usize)) -> Self {
        Logger::new(path, code, row, col, 0, LogType::Info)
    }

    /// Add message to an existing log
    pub fn attach_message<T: AsRef<str>>(mut self, text: T) -> Self {
        self.message = Some(String::from(text.as_ref()));
        self
    }

    /// Add comment to an existing log
    pub fn attach_comment<T: AsRef<str>>(mut self, text: T) -> Self {
        self.comment = Some(String::from(text.as_ref()));
        self
    }

    /// Add code to an existing log.
    /// This code will be used to display a snippet where the message was triggered.
    pub fn attach_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Shows (renders) the message while giving 
    /// the ownership to this object away
    pub fn show(self) -> Self {
        let color = match &self.kind {
            LogType::Error => (255, 80, 80),
            LogType::Warning => (255, 180, 80),
            LogType::Info => (80, 80, 255)
        };
        // If this error is based in code
        if self.row > 0 && self.col > 0 {
            Displayer::new(color, self.row, self.col, self.len)
                .header(self.kind.clone())
                .text(self.message.clone())
                .path(self.path.clone())
                .padded_text(self.comment.clone())
                .snippet(self.code.clone())
        }
        // If this error is a message error
        else {
            Displayer::new(color, self.row, self.col, self.len)
                .header(self.kind.clone())
                .text(self.message.clone())
                .padded_text(self.comment.clone());
        }
        self
    }

    /// Exit current process with error code 1
    pub fn exit(self) {
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    
    #[test]
    fn test_logger() {
        // use super::Logger;
        // Logger::new_err_msg("This is not a message")
        //     .attach_comment("Or is it?")
        //     .show()
        //     .exit();
    }
}



