mod parser;

fn main() {
    let compounds = vec!["+=", "-="];

    let code = "'test this' + ( 2 - 1 )";
    let lex = parser::lexer(code);
    println!("{:?}", lex);
}
