use heraclitus::{*, patterns::{syntax, number, token}};

struct Number {}
impl SyntaxModule for Number {
    fn new() -> Self {
        Number {  }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        number(meta, vec![])?;
        Ok(())
    }
}

struct Add {
    left: Option<Number>,
    right: Option<Expr>
}
impl SyntaxModule for Add {
    fn new() -> Self {
        Add { left: None, right: None }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        match meta.expr.get(meta.index) {
            Some(_) => {
                let mut  left = Number::new();
                let mut right = Expr::new();
                syntax(meta, &mut left)?;
                self.left = Some(left);
                token(meta, "+")?;
                syntax(meta, &mut right)?;
                self.right = Some(right);
                Ok(())
            }
            None => Err(())
        }
    }
}

enum ExprType {
    Add(Add),
    Number(Number)
}

struct Expr {
    expr: Option<Box<ExprType>>
}
impl Expr {
    fn parse_module(&mut self, meta: &mut SyntaxMetadata, module: ExprType) -> SyntaxResult {
        match module {
            ExprType::Add(mut module) => {
                if let Ok(()) = syntax(meta, &mut module) {
                    self.expr = Some(Box::new(ExprType::Add(module)));
                    return Ok(())
                }
            },
            ExprType::Number(mut module) => {
                if let Ok(()) = syntax(meta, &mut module) {
                    self.expr = Some(Box::new(ExprType::Number(module)));
                    return Ok(())
                }
            }
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

#[test]
fn main() {
    let symbols = vec!['+'];
    let region = reg![
        reg!(string as "string literal" => {
            begin: "'",
            end: "'"
        })
    ];
    let rules = Rules::new(symbols, region);
    let mut compiler = Compiler::new("Arith", rules);
    compiler.load("12.24 +.123 + 12");
    assert!(compiler.compile(Expr { expr: None }).is_ok());
}