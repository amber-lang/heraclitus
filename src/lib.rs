#![warn(missing_docs)]

//! # Heraclitus - the compiler frontend
//! 
//! With heraclitus you can create your language by skipping the cumbersome lexing step
//! and using convenience parsing methods that can get you started on your language much quicker.
//! 
//! The main construct that you need is the `Compiler`. The compiler will tokenize your code and assemble it
//! in a way that you can use to create AST by implementing predefined trait that helps you parse your code.
//! 
//! It's pretty simple. In order to get started you need 3 steps:
//! 1. Create lexing rules
//! 2. Create your ast nodes and let them implement trait provided by this package
//! 3. Create compiler and tie all the components together
//! 
//! Voilá!
//! Now you got yourself a ready to analyze / interpret / validate / compile AST.
//! 
//! Ready to get started?
//! # Example
//! ```
//! use heraclitus::prelude::*;
//! # let rules = Rules::new(vec![], reg![]);
//! Compiler::new("HerbScript", rules);
//! ```
//! It is recommended to use included prelude to import just the things we will actually need.
//! 
//! The `Compiler` requires lexer rules in order to exist.
//! 
//! ```
//! # use heraclitus::prelude::*;
//! # fn compiler() -> Result<(), LexerError> {
//! # let rules = Rules::new(vec![], reg![]);
//! let cc = Compiler::new("HerbScript", rules);
//! let tokens = cc.tokenize()?;
//! # Ok(())
//! # }
//! ```

pub mod rules;
pub mod compiler;

pub mod prelude {
    pub use crate::*;
    pub use crate::rules::*;
    pub use crate::compiler::*;
    pub use crate::compiler::patterns::*;
    pub use crate::compiler::{ErrorDetails, ErrorPosition};
}