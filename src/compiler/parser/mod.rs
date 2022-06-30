mod syntax_module;
mod pattern;
mod preset;

pub use syntax_module::*;
pub mod patterns {
    pub use super::pattern::*;
    pub use super::preset::*;
}