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
            ExprType::Add(md) => self.get(meta, md, |md| ExprType::Add(md)),
            ExprType::Number(md) => self.get(meta, md, |md| ExprType::Number(md))
        }
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