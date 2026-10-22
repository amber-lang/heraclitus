//! Display your errors
//!
//! Logger makes it easy for you to display errors. This sub-module is pretty powerful
//! to be used extensively instead of building own implementation of such mechanism.
//! However, if you need more specific functionality - it is encouraged to create your
//! own implementation of such mechanism.

use crate::compiling::{Metadata, Token};
use crate::compiling::failing::logger::Logger;
use crate::compiling::failing::position_info::PositionInfo;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Type of the message that logger shall display
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MessageType {
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Info message
    Info
}

/// Code snippet that Message can relate to
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Snippet {
    pub(crate) line: usize,
    pub(crate) before: Option<String>,
    pub(crate) current: String,
    pub(crate) after: Option<String>
}

/// A message (error, warning or info) object that can be shown to the user.
/// Optionally it can point to a place in the code.
///
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # let path = "path/to/file";
/// # let position = PositionInfo::at_pos(Some(path), (1, 1), 0, 0);
/// # let meta = DefaultMetadata::new(vec![], "code").with_path(path);
/// let guess = "type";
///
/// Message::new_err_at_position(&meta, position)
///     .message("Type of this parameter is invalid")
///     .comment(format!("Maybe you meant type {guess} instead"))
///     .show();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message {
    /// Type of the message
    pub kind: MessageType,
    /// Trace of the error
    pub trace: Vec<PositionInfo>,
    /// Optionally store source code snippet
    pub snippet: Option<Snippet>,
    /// Optionally store message
    pub message: Option<String>,
    /// Optionally store comment
    pub comment: Option<String>
}

impl Message {
    /// Create a new logger instance
    pub fn new(code: &str, trace: &[PositionInfo], kind: MessageType) -> Self {
        let trace = trace.iter().rev().cloned().collect::<Vec<PositionInfo>>();
        let snippet = trace.first().and_then(|pos| Self::get_snippet(code, pos));
        Message {
            kind,
            trace,
            snippet,
            message: None,
            comment: None
        }
    }

    /// Create a new logger instance with message (suited for messages not related with issues in code)
    pub fn new_msg(message: impl AsRef<str>, kind: MessageType) -> Self {
        Message {
            kind,
            trace: vec![],
            snippet: None,
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

    /* Attach additional information */

    /// Add message to an existing log
    pub fn message(mut self, text: impl AsRef<str>) -> Self {
        self.message = Some(String::from(text.as_ref()));
        self
    }

    /// Add comment to an existing log
    pub fn comment(mut self, text: impl AsRef<str>) -> Self {
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
                .line(self.message.clone())
                .path()
                .snippet(self.snippet.as_ref())
                .line(self.comment.clone());
        }
        // If this error is a message error
        else {
            Logger::new(self.kind.clone(), &self.trace)
                .header(self.kind.clone())
                .line(self.message.clone())
                .padded_line(self.comment.clone());
        }
    }

    fn get_full_trace(meta: &impl Metadata, position: PositionInfo) -> Vec<PositionInfo> {
        let mut trace = meta.get_trace();
        trace.push(position);
        trace
    }

    pub(crate) fn get_snippet(code: &str, position: &PositionInfo) -> Option<Snippet> {
        let (row, _) = position.get_pos_by_file_or_code(code);
        let index = row.checked_sub(1)?;
        let lines = code.lines().map(str::trim_end).collect::<Vec<&str>>();

        Some(Snippet {
            line: row,
            before: index.checked_sub(1).and_then(|i| lines.get(i).map(|v| v.to_string())),
            current: lines.get(index)?.to_string(),
            after: lines.get(index + 1).map(|v| v.to_string())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::{DefaultMetadata, Message, PositionInfo};

    #[test]
    fn test_message() {
        Message::new_err_msg("This is not a message")
            .comment("Or is it?")
            .show();
    }

    #[test]
    fn test_message_with_code() {
        let code = [
            "... some code",
            "name = false",
            "... further code",
        ].join("\n");
        let path = "path/to/file";
        let position = PositionInfo::at_pos(Some(path), (2, 1), 14, 4);
        let guess = "type";
        let meta = DefaultMetadata::new(vec![], code).with_path(path);
        Message::new_err_at_position(&meta, position)
            .message("Type of this parameter is invalid")
            .comment(format!("Maybe you meant type {guess} instead"))
            .show();
    }
}
