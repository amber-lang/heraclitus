use colored::Colorize;
use crate::compiler::Token;

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
    pub message_content: Option<String>,
    pub comment_content: Option<String>
}

impl<'a> Log<'a> {
    pub fn new(path: &'a String, row: usize, col: usize, kind: LogType) -> Self {
        Log {
            kind,
            path,
            row,
            col,
            message_content: None,
            comment_content: None
        }
    }

    pub fn new_err(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Error)
    }

    pub fn new_warn(path: &'a String, row: usize, col: usize) -> Self {
        Log::new(path, row, col, LogType::Warning)
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
        self.message_content = Some(String::from(text.as_ref()));
        self
    }

    pub fn comment<T: AsRef<str>>(mut self, text: T) -> Self {
        self.comment_content = Some(String::from(text.as_ref()));
        self
    }

    pub fn send(&self) {
        match &self.kind {
            LogType::Error => {
                println!("{}", format!(" ERROR ").truecolor(0, 0, 0).bold().on_truecolor(255, 80, 80));
                if let Some(message) = &self.message_content {
                    println!("{}", format!("{}", message).red());
                    println!("{}", format!("at {}:{}:{}", self.path, self.row, self.col).red().dimmed());
                }
                if let Some(comment) = &self.comment_content {
                    println!("\n{}", format!("{}", comment).red());
                }
            },
            LogType::Warning => {
                println!("{}", format!(" WARN ").truecolor(0, 0, 0).bold().on_truecolor(255, 180, 80));
                if let Some(message) = &self.message_content {
                    println!("{}", format!("{}", message).yellow());
                    println!("{}", format!("at {}:{}:{}", self.path, self.row, self.col).yellow().dimmed());
                }
                if let Some(comment) = &self.comment_content {
                    println!("\n{}", format!("{}", comment).yellow());
                }
            },
            LogType::Info => {
                println!("{}", format!(" INFO ").truecolor(0, 0, 0).bold().on_blue());
                if let Some(message) = &self.message_content {
                    println!("{}", format!("{}", message).blue());
                    println!("{}", format!("at {}:{}:{}", self.path, self.row, self.col).blue().dimmed());
                }
                if let Some(comment) = &self.comment_content {
                    println!("\n{}", format!("{}", comment).blue());
                }
            }
        };
    }
}




