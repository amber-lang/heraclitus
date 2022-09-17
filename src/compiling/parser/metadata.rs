use crate::compiling::Token;

/// Default implementation of metadata. 
/// This is useful for debuging or languages that are not too demanding.
pub struct DefaultMetadata {
    /// Current index in the token stream
    pub index: usize,
    /// Lexem of tokens to parse
    pub expr: Vec<Token>,
    /// Optionally path of the file
    pub path: Option<String>,
    /// Optionally code of the file
    pub code: Option<String>,
    /// Debug value that is used internally
    pub indent: Option<usize>
}

impl Metadata for DefaultMetadata {
    fn new(tokens: Vec<Token>, path: Option<String>, code: Option<String>) -> Self {
        DefaultMetadata {
            index: 0,
            expr: tokens,
            path,
            code,
            indent: None
        }
    }

    fn get_token_at(&self, index: usize) -> Option<Token> {
        self.expr.get(index).cloned()
    }
    
    fn set_index(&mut self, index: usize) {
        self.index = index
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_debug(&mut self) -> Option<usize> {
        self.indent
    }

    fn set_debug(&mut self, indent: usize) {
       self.indent = Some(indent)
    }

    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }
}

/// Metadata for carrying information through the ASI parsing phases.
/// 
/// This Metadata trait should define your metadata struct with all additional data that you need
/// in order to parse the source code. If you are looking for the default implementation - look at the `DefaultMetadata`.
pub trait Metadata {
    /// Load tokens into metadata
    fn new(tokens: Vec<Token>, path: Option<String>, code: Option<String>) -> Self;
    /// Return optionally token under desired index in the lexem
    fn get_token_at(&self, index: usize) -> Option<Token>;
    /// Get current index
    fn get_index(&self) -> usize;
    /// Set current index
    fn set_index(&mut self, index: usize);
    /// Getter for debug value (debug value should be Option<usize> initialized with None)
    fn get_debug(&mut self) -> Option<usize>;
    /// Setter for debug value
    fn set_debug(&mut self, indent: usize);
    /// Getter for path of the file
    fn get_path(&self) -> Option<String>;
    /// Getter for code of the file
    fn get_code(&self) -> Option<&String>;
    /// Optionally set logic of incrementing the index number
    fn increment_index(&mut self) {
        let index = self.get_index();
        self.set_index(index + 1);
    }
    /// Return token under current index
    fn get_current_token(&self) -> Option<Token> {
        let index = self.get_index();
        self.get_token_at(index)
    }
    /// Change current index by given offset
    fn offset_index(&mut self, offset: usize) {
        let index = self.get_index();
        self.set_index(index + offset);
    }
}