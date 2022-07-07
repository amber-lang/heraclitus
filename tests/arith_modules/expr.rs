use heraclitus::prelude::*;
use super::*;

pub enum ExprType {
    Add(Add),
    Number(Number)
}

pub struct Expr {
    expr: Option<Box<ExprType>>
}
impl Expr {
    fn get<M,S>(&mut self, meta: &mut M, mut module: S, cb: impl Fn(S) -> ExprType) -> SyntaxResult
    where
        M: Metadata,
        S: SyntaxModule<M>
    {
        if let Ok(()) = syntax(meta, &mut module) {
            self.expr = Some(Box::new(cb(module)));
            Ok(())
        } else { Err(()) }
    }
    fn parse_module(&mut self, meta: &mut SyntaxMetadata, module: ExprType) -> SyntaxResult {
        match module {
            ExprType::Add(md) => if let Ok(()) = self.get(meta, md, |md| ExprType::Add(md)) { return Ok(()) },
            ExprType::Number(md) => if let Ok(()) = self.get(meta, md, |md| ExprType::Number(md)) { return Ok(()) },
        }
        Err(())
    }
}
impl SyntaxModule<SyntaxMetadata> for Expr {
    fn new() -> Self {
        Expr { expr: None }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        let modules: Vec<ExprType> = vec![
            ExprType::Add(Add::new()),
            ExprType::Number(Number::new())
        ];
        for module in modules {
            if let Ok(()) = self.parse_module(meta, module) {
                if let Some(tok) = meta.expr.get(meta.index) {
                    if tok.word != "\n" {
                        return Err(())
                    }
                }
                return Ok(())
            }
        }
        Err(())
    }
}