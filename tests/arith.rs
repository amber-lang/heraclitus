use heraclitus_compiler::prelude::*;
mod arith_modules;

#[test]
fn arith() {
    let symbols = vec!['+', '/'];
    let region = reg![
        reg!(string as "string literal" => {
            begin: "'",
            end: "'"
        }),
        reg!(comment as "comment line" => {
            begin: "//",
            end: "\n"
        })
    ];
    let rules = Rules::new(symbols, vec![], region);
    let mut compiler = Compiler::new("Arith", rules);
    compiler.load("// test\n12.24 +.123 + 12 + 321");
    let mut expr = arith_modules::Expr::new();
    compiler.debug();
    let lexem = compiler.tokenize().unwrap();
    let mut meta = DefaultMetadata::new(lexem, compiler.path.clone(), compiler.code.clone());
    assert!(expr.parse_debug(&mut meta).is_ok());
}