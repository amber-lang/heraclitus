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
        syntax(meta, Number {})?;
        token(meta, "+")?;
        syntax(meta, Number {})?;
        Ok(())
    }
}

struct Expr {}
impl SyntaxModule for Expr {
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        syntax(meta, Add {})?;
        Ok(())
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
    assert!(compiler.compile(Expr {}).is_ok());
}