//! Module for compiling your language
//! 
//! This module supplies you with many features, among them - `Compiler`
//! that helps you tokenize your code or even parse it entirely.

mod lexing;

mod compiler;
mod token;
mod parser;
pub mod failing;

pub use lexing::*;
pub use compiler::*;
pub use token::*;
pub use parser::*;
