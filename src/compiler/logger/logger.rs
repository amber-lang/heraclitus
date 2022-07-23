//! Display your errors
//! 
//! Logger makes it easy for you to display errors. This sub-module is pretty powerful
//! to be used extensively instead of building own implementation of such mechanism.
//! However, if you need more specific functionality - it is encouraged to create your
//! own implementation of such mechanism.

#![allow(dead_code)]
use std::process;
use super::displayer::Displayer;

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
/// # let path = format!("path/to/file");
/// # let position = (0, 0);
/// # let guess = "type";
/// # let code = format!("code");
/// Logger::new_err(path, position)
///     .attach_message("Type of this parameter is invalid")
///     .attach_comment(format!("Maybe you meant type {guess} instead"))
///     .attach_code(code)
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
    /// Path to the source file
    pub path: String,
    /// Optionally store source code
    pub code: Option<String>,
    /// Optionally store message
    pub message: Option<String>,
    /// Optionally store comment
    pub comment: Option<String>
}

impl Logger {
    /// Create a new logger instance
    pub fn new(path: String, row: usize, col: usize, kind: LogType) -> Self {
        Logger {
            kind,
            path,
            row,
            col,
            code: None,
            message: None,
            comment: None
        }
    }

    /// Create an error by supplying essential information about the location
    pub fn new_err(path: String, (row, col): (usize, usize)) -> Self {
        Logger::new(path, row, col, LogType::Error)
    }

    /// Create a warning by supplying essential information about the location
    pub fn new_warn(path: String, (row, col): (usize, usize)) -> Self {
        Logger::new(path, row, col, LogType::Warning)
    }

    /// Create an info by supplying essential information about the location
    pub fn new_info(path: String, (row, col): (usize, usize)) -> Self {
        Logger::new(path, row, col, LogType::Info)
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
        Displayer::new(color, self.row, self.col)
            .header(self.kind.clone())
            .text(self.message.clone())
            .path(&self.path)
            .padded_text(self.comment.clone());
        self
    }

    /// Exit current process with error code 1
    pub fn exit(self) {
        process::exit(1);
    }
}




