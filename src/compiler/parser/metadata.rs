use crate::compiler::Token;

/// Default implementation of metadata. 
/// This is useful for debuging or languages that are not too demanding.
pub struct DefaultMetadata {
    /// Current index in the token stream
    pub index: usize,
    /// Lexem of tokens to parse
    pub expr: Vec<Token>,
    /// Optionally path of the file
    pub path: Option<String>
}

impl Metadata for DefaultMetadata {
    fn new(expression: Vec<Token>, path: Option<String>) -> Self {
        DefaultMetadata {
            index: 0,
            expr: expression,
            path
        }
    }
    fn get_token_at(&self, index: usize) -> Option<Token> {
        if let Some(token) = self.expr.get(index) {
            return Some(token.clone())
        } else { None }
    }
    fn set_index(&mut self, index: usize) {
        self.index = index
    }
    fn get_index(&self) -> usize {
        self.index
    }
}

/// Metadata for carrying information through the ASI parsing phases.
/// 
/// This Metadata trait should define your metadata struct with all additional data that you need
/// in order to parse the source code. If you are looking for the default implementation - look at the `DefaultMetadata`.
pub trait Metadata {
    /// Create a new metadata from lexem and a path (used by Compiler::compile)
    fn new(expression: Vec<Token>, path: Option<String>) -> Self;
    /// Return optionally token under desired index in the lexem
    fn get_token_at(&self, index: usize) -> Option<Token>;
    /// Get current index
    fn get_index(&self) -> usize;
    /// Set current index
    fn set_index(&mut self, index: usize);
    /// Optionally set logic of incrementing the index number
    fn increment_index(&mut self) {
        let index = self.get_index();
        self.set_index(index + 1);
    }
}