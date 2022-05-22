use crate::compiler::Compiler;
use super::region_handler::RegionHandler;

pub fn lexer(cc: &Compiler) {
    let rules = RegionHandler::new(&cc.rules);
    for letter in cc.code.chars() {
        let new_rule = rules.handle_region();
        // TODO: When handling symbol, use peekable
        // to handle the self.add_word(True)
        // https://doc.rust-lang.org/std/iter/struct.Peekable.html
    }
}