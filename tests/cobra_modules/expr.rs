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
    fn get<M,S>(&mut self, meta: &mut M, mut module: S, cb: impl Fn(S) -> ExprType) -> SyntaxResult
    where
        M: Metadata,
        S: SyntaxModule<M>
    {
        match syntax(meta, &mut module) {
            Ok(()) => {
                self.expr = Some(Box::new(cb(module)));
                Ok(())    
            }
            Err(details) => Err(details)
        }
    }
    fn parse_module(&mut self, meta: &mut SyntaxMetadata, module: ExprType) -> SyntaxResult {
        match module {
            ExprType::Text(md) => self.get(meta, md, |md| ExprType::Text(md))
        }
    }
}
impl SyntaxModule<SyntaxMetadata> for Expr {
    fn new() -> Self {
        Expr { expr: None }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        let modules: Vec<ExprType> = vec![
            ExprType::Text(Text::new())
        ];
        let mut err = None;
        for module in modules {
            match self.parse_module(meta, module) {
                Ok(()) => return Ok(()),
                Err(details) => {
                    err = Some(details);
                }
            }
        }
        Err(err.unwrap())
    }
}