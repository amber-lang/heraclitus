use heraclitus::prelude::*;
mod arith_modules;

#[test]
fn arith() {
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
    assert!(compiler.compile(&mut arith_modules::Expr::new()).is_ok());
}