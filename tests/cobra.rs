use heraclitus_compiler::prelude::*;
mod cobra_modules;

#[test]
fn cobra() {
    let symbols = vec!['+'];
    let compounds = vec![('+', '+')];
    let region = reg![
        reg!(string as "string literal" => {
            begin: "'",
            end: "'"
        })
    ];
    let rules = Rules::new(symbols, compounds, region);
    let mut compiler = Compiler::new("Cobra", rules);
    compiler.use_indents();
    compiler.load(vec![
        "if 'condition\\\\':",
        "  'do + this'",
        "  'do ++ that'"
    ].join("\n"));
    let mut ast = cobra_modules::IfStatement::new();
    compiler.debug();
    let lexem = compiler.tokenize().unwrap();
    let mut meta = DefaultMetadata::new(lexem, compiler.path.clone(), compiler.code.clone());
    assert!(ast.parse_debug(&mut meta).is_ok());
}
