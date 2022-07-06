use heraclitus::prelude::*;
mod cobra_modules;

#[test]
fn cobra() {
    let symbols = vec!['+'];
    let region = reg![
        reg!(string as "string literal" => {
            begin: "'",
            end: "'"
        })
    ];
    let rules = Rules::new(symbols, region);
    let mut compiler = Compiler::new("Arith", rules);
    compiler.use_indents();
    compiler.load(vec![
        "if 'condition':",
        "  'do this'",
        "  'do that'"
    ].join("\n"));
    let mut ast = cobra_modules::IfStatement::new();
    assert!(compiler.compile(&mut ast).is_ok());
}