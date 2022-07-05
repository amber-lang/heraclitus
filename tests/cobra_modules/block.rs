use heraclitus::prelude::*;
use super::*;

#[derive(Debug)]
pub struct Block {
    statements: Vec<Expr>,
    indent_size: usize
}
impl Block {
    pub fn set_indent_size(&mut self, size: usize) {
        self.indent_size = size;
    }
}
impl SyntaxModule for Block {
    fn new() -> Self {
        Block { statements: vec![], indent_size: 0 }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        loop {
            let mut expr = Expr::new();
            if let Ok(()) = syntax(meta, &mut expr) {
                self.statements.push(expr);
            } else {
                return Ok(())
            }
        }
    }
}