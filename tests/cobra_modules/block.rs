use std::cmp::Ordering;
use heraclitus_compiler::prelude::*;
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
impl SyntaxModule<DefaultMetadata> for Block {
    syntax_name!("Block");
    fn new() -> Self {
        Block { statements: vec![], indent_size: 0 }
    }
    fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
        loop {
            if let Ok(cmp) = indent_with(meta, self.indent_size) {
                match cmp {
                    Ordering::Less => return Ok(()),
                    Ordering::Equal => {},
                    Ordering::Greater => return Err(ErrorDetails::from_metadata(meta))
                }
            }
            let mut expr = Expr::new();
            if let Ok(()) = syntax(meta, &mut expr) {
                self.statements.push(expr);
            } else {
                return Ok(())
            }
        }
    }
}