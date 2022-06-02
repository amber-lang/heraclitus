use crate::compiler::Token;

pub enum LogType {
    Err,
    Warn,
    Info
}

pub struct Log<'a> {
    pub kind: LogType,
    pub row: usize,
    pub col: usize,
    pub path: &'a String,
    pub message: Option<String>,
    pub comment: Option<String>
}

impl<'a> Log<'a> {
    pub fn new(path: &'a String, row: usize, col: usize, kind: LogType) -> Self {
        Log {
            kind,
            path,
            row,
            col,
            message: None,
            comment: None
        }
    }

    pub fn new_err(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Err)
    }

    pub fn new_warn(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Warn)
    }

    pub fn new_info(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Info)
    }

    pub fn new_err_at_token(token: Token<'a>) -> Self {
        Log::new_err(token.path, token.pos.0, token.pos.1)
    }

    pub fn new_warn_at_token(token: Token<'a>) -> Self {
        Log::new_warn(token.path, token.pos.0, token.pos.1)
    }

    pub fn new_info_at_token(token: Token<'a>) -> Self {
        Log::new_info(token.path, token.pos.0, token.pos.1)
    }

    pub fn message<T: AsRef<str>>(mut self, text: T) -> Self {
        self.message = Some(String::from(text.as_ref()));
        self
    }

    pub fn comment<T: AsRef<str>>(mut self, text: T) -> Self {
        self.comment = Some(String::from(text.as_ref()));
        self
    }
}




