//! Display your errors
//! 
//! Logger makes it easy for you to display errors. This sub-module is pretty powerful
//! to be used extensively instead of building own implementation of such mechanism.
//! However, if you need more specific functionality - it is encouraged to create your
//! own implementation of such mechanism.

#![allow(dead_code)]
use crate::compiling::{Metadata, Token};
use crate::compiling::failing::logger::Logger;
use crate::compiling::failing::position_info::PositionInfo;

/// Type of the message that logger shall display
#[derive(Clone, Debug)]
pub enum MessageType {
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
/// # let position = PositionInfo::at_pos(path.clone(), (0, 0), 0);
/// # let guess = "type";
/// # let code = Some(format!("code"));
/// # let mut meta = DefaultMetadata::new(vec![], path, code);
/// Message::new_err_at_position(&mut meta, position)
///     .message("Type of this parameter is invalid")
///     .comment(format!("Maybe you meant type {guess} instead"))
///     .show();
/// ```
#[derive(Debug, Clone)]
pub struct Message {
    /// Type of the message
    pub kind: MessageType,
    /// Trace of the error
    pub trace: Vec<PositionInfo>,
    /// Optionally store source code
    pub code: Option<String>,
    /// Optionally store message
    pub message: Option<String>,
    /// Optionally store comment
    pub comment: Option<String>
}

impl Message {
    /// Create a new logger instance
    pub fn new(code: Option<&String>, trace: &[PositionInfo], kind: MessageType) -> Self {
        Message {
            kind,
            trace: trace.iter().rev().cloned().collect(),
            code: code.cloned(),
            message: None,
            comment: None
        }
    }

    /// Create a new logger instance with message (suited for messages not related with issues in code)
    pub fn new_msg(message: impl AsRef<str>, kind: MessageType) -> Self {
        Message {
            kind,
            trace: vec![],
            code: None,
            message: Some(message.as_ref().to_string()),
            comment: None
        }
    }

    /// Create a new error instance at given token position if possible
    pub fn new_at_token(meta: &impl Metadata, token: Option<Token>, kind: MessageType) -> Self {
        Self::new(meta.get_code(), &Self::get_full_trace(meta, PositionInfo::from_token(meta, token)), kind)
    }

    /* New Error Message */

    /// Show error message that does not relate to code
    pub fn new_err_msg(message: impl AsRef<str>) -> Self {
        Self::new_msg(message, MessageType::Error)
    }

    /// Show warning message that does not relate to code
    pub fn new_warn_msg(message: impl AsRef<str>) -> Self {
        Self::new_msg(message, MessageType::Warning)
    }

    /// Show info message that does not relate to code
    pub fn new_info_msg(message: impl AsRef<str>) -> Self {
        Self::new_msg(message, MessageType::Info)
    }

    /* New Error Message at Token */

    /// Show error message based on the token
    pub fn new_err_at_token(meta: &impl Metadata, token: Option<Token>) -> Self {
        Self::new_at_token(meta, token, MessageType::Error)
    }

    /// Show warning message based on the token
    pub fn new_warn_at_token(meta: &impl Metadata, token: Option<Token>) -> Self {
        Self::new_at_token(meta, token, MessageType::Warning)
    }

    /// Show info message based on the token
    pub fn new_info_at_token(meta: &impl Metadata, token: Option<Token>) -> Self {
        Self::new_at_token(meta, token, MessageType::Info)
    }

    /* New Error Message at Position */

    /// Create an error by supplying essential information about the location
    pub fn new_err_at_position(meta: &impl Metadata, pos: PositionInfo) -> Self {
        Self::new(meta.get_code(), &Self::get_full_trace(meta, pos), MessageType::Error)
    }

    /// Create a warning by supplying essential information about the location
    pub fn new_warn_at_position(meta: &impl Metadata, pos: PositionInfo) -> Self {
        Self::new(meta.get_code(), &Self::get_full_trace(meta, pos), MessageType::Warning)
    }

    /// Create an info by supplying essential information about the location
    pub fn new_info_at_position(meta: &impl Metadata, pos: PositionInfo) -> Self {
        Self::new(meta.get_code(), &Self::get_full_trace(meta, pos), MessageType::Info)
    }

    /* Attach additional infromation */

    /// Add message to an existing log
    pub fn message<T: AsRef<str>>(mut self, text: T) -> Self {
        self.message = Some(String::from(text.as_ref()));
        self
    }

    /// Add comment to an existing log
    pub fn comment<T: AsRef<str>>(mut self, text: T) -> Self {
        self.comment = Some(String::from(text.as_ref()));
        self
    }

    /// Shows (renders) the message while giving 
    /// the ownership to this object away
    pub fn show(&self) {
        // If this error is based in code
        if !self.trace.is_empty() {
            Logger::new(self.kind.clone(), &self.trace)
                .header(self.kind.clone())
                .text(self.message.clone())
                .path()
                .padded_text(self.comment.clone())
                .snippet(self.code.clone())
        }
        // If this error is a message error
        else {
            Logger::new(self.kind.clone(), &self.trace)
                .header(self.kind.clone())
                .text(self.message.clone())
                .padded_text(self.comment.clone());
        }
    }

    fn get_full_trace(meta: &impl Metadata, position: PositionInfo) -> Vec<PositionInfo> {
        let mut trace = meta.get_trace();
        trace.push(position);
        trace
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



