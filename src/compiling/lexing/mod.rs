//! Lexer module
//!
//! This module holds all the lexer related modules

use crate::prelude::PositionInfo;

mod compound_handler;
#[cfg(feature = "lexer_dynamic")]
pub mod lexer;
#[cfg(feature = "lexer_static")]
pub mod lexer_static;
mod reader;
mod region_handler;

/// Lexer's error type
#[derive(Debug)]
pub enum LexerErrorType {
    /// Unspillable region has been spilled
    Singleline,
    /// Given region left unclosed
    Unclosed,
}

/// Type containing full error of lexer
pub type LexerError = (LexerErrorType, PositionInfo);
