use heraclitus::prelude::*;
use super::*;

#[derive(Clone)]
pub enum ExprId {
    Add = 0,
    Number
}

pub enum ExprType {
    Add(Add),
    Number(Number)
}

pub struct Expr {
    expr: Option<Box<ExprType>>,
    excludes: Option<ExprId>
}
impl Expr {
    pub fn exclude(&mut self, target: ExprId) {
        self.excludes = Some(target);
    }
    fn get<M,S>(&mut self, meta: &mut M, mut module: S, cb: fn(S) -> ExprType, id: ExprId) -> SyntaxResult
    where
        M: Metadata,
        S: SyntaxModule<M>
    {
        // Check if exclusion occurs
        if let Some(excludes) = &self.excludes {
            if excludes.clone() as usize != id as usize {
                // Match syntax
                if let Ok(()) = syntax(meta, &mut module) {
                    self.expr = Some(Box::new(cb(module)));
                    return Ok(())
                }
            }
        }
        Err(())
    }
    fn parse_module(&mut self, meta: &mut SyntaxMetadata, module: ExprType) -> SyntaxResult {
        match module {
            ExprType::Add(md) => if let Ok(()) = self.get(meta, md, ExprType::Add, ExprId::Add) { return Ok(()) },
            ExprType::Number(md) => if let Ok(()) = self.get(meta, md, ExprType::Number, ExprId::Number) { return Ok(()) },
        }
        Err(())
    }
}

impl SyntaxModule<SyntaxMetadata> for Expr {
    fn new() -> Self {
        Expr { expr: None, excludes: None }
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