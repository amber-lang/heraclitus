use heraclitus::prelude::*;
use super::text::*;

#[derive(Debug)]
pub enum ExprType {
    Text(Text)
}

#[derive(Debug)]
pub struct Expr {
    expr: Option<Box<ExprType>>
}
impl Expr {
    fn get<T: SyntaxModule>(&mut self, meta: &mut SyntaxMetadata, mut module: T, cb: fn(T) -> ExprType) -> SyntaxResult {
        if let Ok(()) = syntax(meta, &mut module) {
            self.expr = Some(Box::new(cb(module)));
            Ok(())
        } else { Err(()) }
    }
    fn parse_module(&mut self, meta: &mut SyntaxMetadata, module: ExprType) -> SyntaxResult {
        match module {
            ExprType::Text(md) => if let Ok(()) = self.get(meta, md, |md| ExprType::Text(md)) { return Ok(()) },
        }
        Err(())
    }
}
impl SyntaxModule for Expr {
    fn new() -> Self {
        Expr { expr: None }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        let modules: Vec<ExprType> = vec![
            ExprType::Text(Text::new()),
        ];
        for module in modules {
            if let Ok(()) = self.parse_module(meta, module) {
                return Ok(())
            }
        }
        Err(())
    }
}