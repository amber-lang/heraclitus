use heraclitus::prelude::*;
use super::*;

#[derive(Debug)]
pub struct Add {
    left: Option<Expr>,
    right: Option<Expr>
}

impl SyntaxModule<DefaultMetadata> for Add {
    fn new() -> Self {
        Add {
            left: None,
            right: None
        }
    }
    fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
        match meta.expr.get(meta.index) {
            Some(_) => {
                let mut  left = Expr::new();
                let mut right = Expr::new();
                left.exclude(ExprId::Add);
                syntax(meta, &mut left)?;
                self.left = Some(left);
                token(meta, "+")?;
                syntax(meta, &mut right)?;
                self.right = Some(right);
                Ok(())
            }
            None => Err(ErrorDetails::with_eof())
        }
    }
}