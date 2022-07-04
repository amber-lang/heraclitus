use std::process;
use crate::compiler::Token;
use super::displayer::Displayer;

#[derive(Clone)]
pub enum LogType {
    Error,
    Warning,
    Info
}

pub struct Log<'a> {
    pub kind: LogType,
    pub row: usize,
    pub col: usize,
    pub path: &'a String,
    pub code: Option<&'a String>,
    pub message: Option<String>,
    pub comment: Option<String>
}

impl<'a> Log<'a> {
    /// Create a new logger instance
    pub fn new(path: &'a String, row: usize, col: usize, kind: LogType) -> Self {
        Log {
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
    pub fn new_err(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Error)
    }

    /// Create a warning by supplying essential information about the location
    pub fn new_warn(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Warning)
    }

    /// Create an info by supplying essential information about the location
    pub fn new_info(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Info)
    }

    /// Create an error using a token to supply essential information
    pub fn new_err_at_token(token: Token<'a>) -> Self {
        Log::new_err(token.path, token.pos.0, token.pos.1)
    }

    /// Create a warning using a token to supply essential information
    pub fn new_warn_at_token(token: Token<'a>) -> Self {
        Log::new_warn(token.path, token.pos.0, token.pos.1)
    }

    /// Create an info using a token to supply essential information
    pub fn new_info_at_token(token: Token<'a>) -> Self {
        Log::new_info(token.path, token.pos.0, token.pos.1)
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

    /// Add code to an existing log
    pub fn attach_code(mut self, code: &'a String) -> Self {
        self.code = Some(code);
        self
    }

    /// Sends (renders) the message while giving 
    /// the ownership to this object away
    pub fn send(self) -> Self {
        let color = match &self.kind {
            LogType::Error => (255, 80, 80),
            LogType::Warning => (255, 180, 80),
            LogType::Info => (80, 80, 255)
        };
        Displayer::new(color, self.row, self.col)
            .header(self.kind.clone())
            .text(self.message.clone())
            .path(self.path)
            .padded_text(self.comment.clone());
        self
    }

    /// Exit current process
    pub fn exit(self) {
        process::exit(1);
    }
}




