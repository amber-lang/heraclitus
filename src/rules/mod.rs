//! Rules for lexing
//! 
//! This module serves an interface objects that can help you create your own material for parsing.
//! Here you can specify how the lexer should tokenize code by using `Rules` struct that is required by compiler

#[macro_use]
mod rules;
mod region;

pub use rules::*;
pub use region::*;