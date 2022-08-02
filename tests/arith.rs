use heraclitus_compiler::prelude::*;
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
    let rules = Rules::new(symbols, vec![], region);
    let mut compiler = Compiler::new("Arith", rules);
    compiler.load("12.24 +.123 + 12 + 321");
    let mut expr = arith_modules::Expr::new();
    assert!(compiler.compile(&mut expr).is_ok());
}