use crate::compiler::Compiler;
use super::region_handler::RegionHandler;
use super::reader::Reader;

// This is just an estimation of token amount
// inside of a typical 200-lined file.
const AVG_TOKEN_AMOUNT: usize = 1024;

struct Lexer<'a> {
    region: RegionHandler,
    reader: Reader<'a>,
    lexem: Vec<String>,
    word: String
}

impl<'a> Lexer<'a> {
    fn new(cc: &'a Compiler) -> Self {
        Lexer {
            region: RegionHandler::new(&cc.rules),
            reader: Reader::new(&cc.code),
            lexem: Vec::with_capacity(AVG_TOKEN_AMOUNT),
            word: String::new()
        }
    }

    fn add_word(&mut self) {
        if self.word.len() > 0 {
            // self.lexem.push()
            // TODO: Finish add_word
        }
    }

    fn is_region(&self, is_matched: bool) -> bool {
        if let Some(region) = self.region.get_region() {
            region.preserve && !is_matched
        }
        else { false }
    }

    pub fn run(mut self) {
        while let Some(letter) = self.reader.next() {
            let is_matched = self.region.handle_region(&self.reader);
            if self.is_region(is_matched) {
                self.word.push(letter);
            }
            else {
                if vec![' ', '\t'].contains(&letter) {
                    // self.add_word()
                }
            }
        }
    
        // TODO: When handling symbol, use peekable
        // to handle the self.add_word(True)
        // https://doc.rust-lang.org/stdx/iter/struct.Peekable.html
    }
}
