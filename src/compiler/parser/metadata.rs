use crate::compiler::Token;

pub struct SyntaxMetadata {
    pub index: usize,
    pub expr: Vec<Token>,
    pub path: Option<String>
}

impl Metadata for SyntaxMetadata {
    fn new(expression: Vec<Token>, path: Option<String>) -> Self {
        SyntaxMetadata {
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

pub trait Metadata {
    fn new(expression: Vec<Token>, path: Option<String>) -> Self;
    fn get_token_at(&self, index: usize) -> Option<Token>;
    fn get_index(&self) -> usize;
    fn set_index(&mut self, index: usize);
}