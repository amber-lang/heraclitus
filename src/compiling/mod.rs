//! Module for compiling your language
//! 
//! This module supplies you with many features, among them - `Compiler`
//! that helps you tokenize your code or even parse it entirely.

mod lexing;

#[cfg(feature = "compiler")]
mod compiler;
mod token;
mod parser;
pub mod failing;

pub use lexing::*;
#[cfg(feature = "compiler")]
pub use compiler::*;
pub use token::*;
pub use parser::*;
