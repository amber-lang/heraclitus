use heraclitus::prelude::*;
use super::*;

#[derive(Clone, Debug)]
pub enum ExprId {
    Add = 0,
    Number
}

#[derive(Debug)]
pub enum ExprType {
    Add(Add),
    Number(Number)
}

#[derive(Debug)]
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
            if excludes.clone() as usize == id as usize {
                return Err(ErrorDetails::from_metadata(meta))
            }
        }
        // Match syntax
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
            ExprType::Add(md) => self.get(meta, md, ExprType::Add, ExprId::Add),
            ExprType::Number(md) => self.get(meta, md, ExprType::Number, ExprId::Number)
        }
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