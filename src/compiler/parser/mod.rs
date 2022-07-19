mod syntax_module;
mod pattern;
mod preset;
mod metadata;

pub use syntax_module::*;
pub use pattern::*;
pub use metadata::*;
pub mod patterns {
    //! Utility functions that help you parse tokens
    //! 
    //! Functions in this module can help you handle tokens in the parsing phase.
    pub use super::pattern::*;
    pub use super::preset::*;
}