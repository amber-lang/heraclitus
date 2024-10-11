//! Lexer module
//!
//! This module holds all the lexer related modules

use crate::prelude::PositionInfo;

mod compound_handler;
pub mod lexer;
mod reader;
mod region_handler;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Lexer's error type
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LexerErrorType {
    /// Unspillable region has been spilled
    Singleline,
    /// Given region left unclosed
    Unclosed,
}

/// Type containing full error of lexer
pub type LexerError = (LexerErrorType, PositionInfo);
