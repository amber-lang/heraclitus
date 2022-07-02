use heraclitus::{*, patterns::{syntax, number, token}};

struct Number {}
impl SyntaxModule for Number {
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        number(meta, vec![])?;
        Ok(())
    }
}

struct Add {}
impl SyntaxModule for Add {
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        match meta.expr.get(meta.index) {
            Some(_) => {
                syntax(meta, Box::new(Number {}))?;
                token(meta, "+")?;
                syntax(meta, Box::new(Number {}))?;
                Ok(())
            }
            None => Err(())
        }
    }
}

struct Expr {
    expr: Option<Box<dyn SyntaxModule>>
}
impl SyntaxModule for Expr {
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        let modules: Vec<Box<dyn SyntaxModule>> = vec![
            Box::new(Add{}),
            Box::new(Number{})
        ];
        for module in modules {
            if let Ok(module) = syntax(meta, module) {
                self.expr = Some(module);
                return Ok(());
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
    compiler.load("12.24 +.123");
    assert!(compiler.compile(Expr { expr: None }).is_ok());
}