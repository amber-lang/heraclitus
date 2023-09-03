use heraclitus_compiler::prelude::*;
use super::{Expr, Block};

#[derive(Debug)]
pub struct IfStatement {
    cond: Expr,
    block: Block
}
impl SyntaxModule<DefaultMetadata> for IfStatement {
    syntax_name!("If");
    fn new() -> Self {
        IfStatement { cond: Expr::new(), block: Block::new() }
    }
    fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
        dbg!("IfStatement");
        token(meta, "if")?;
        syntax(meta, &mut self.cond)?;
        token(meta, ":")?;
        let size = indent(meta)?;
        println!("size: {size}");
        self.block.set_indent_size(size);
        meta.index -= 1;
        syntax(meta, &mut self.block)?;
        Ok(())
    }
}