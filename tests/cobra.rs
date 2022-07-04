use heraclitus::{*, patterns::{syntax, number, token, token_by}};

struct Number {}
impl SyntaxModule for Number {
    fn new() -> Self {
        Number {  }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        token_by(meta, |word| word.starts_with('\'') && word.ends_with('\''))?;
        Ok(())
    }
}

struct String {
    left: Option<Number>,
    right: Option<Expr>
}
impl SyntaxModule for String {
    fn new() -> Self {
        String { left: None, right: None }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        
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
    fn get<T: SyntaxModule>(&mut self, meta: &mut SyntaxMetadata, mut module: T, cb: fn(T) -> ExprType) -> SyntaxResult {
        if let Ok(()) = syntax(meta, &mut module) {
            self.expr = Some(Box::new(cb(module)));
            Ok(())
        } else { Err(()) }
    }
    fn parse_module(&mut self, meta: &mut SyntaxMetadata, module: ExprType) -> SyntaxResult {
        match module {
            ExprType::Add(md) => if let Ok(()) = self.get(meta, md, |md| ExprType::Add(md)) { return Ok(()) },
            ExprType::Number(md) => if let Ok(()) = self.get(meta, md, |md| ExprType::Number(md)) { return Ok(()) },
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
    compiler.load("'this is a test'");
    println!("{:?}", compiler.tokenize());
    // assert!(compiler.compile(Expr { expr: None }).is_ok());
}