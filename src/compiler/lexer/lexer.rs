use crate::compiler::{ Compiler, Token };
use crate::rules::{ Region, Rules };
use super::region_handler::RegionHandler;
use super::reader::Reader;

// This is just an estimation of token amount
// inside of a typical 200-lined file.
const AVG_TOKEN_AMOUNT: usize = 1024;

pub struct Lexer<'a> {
    symbols: &'a Vec<char>,
    region: RegionHandler,
    reader: Reader<'a>,
    lexem: Vec<Token<'a>>,
    path: &'a String
}

impl<'a> Lexer<'a> {
    pub fn new(cc: &'a Compiler) -> Self {
        Lexer {
            symbols: &cc.rules.symbols,
            region: RegionHandler::new(&cc.rules),
            reader: Reader::new(&cc.code),
            lexem: Vec::with_capacity(AVG_TOKEN_AMOUNT),
            path: &cc.path
        }
    }

    fn add_word(&mut self, word: String) -> String {
        if word.len() > 0 {
            let (row, col) = self.reader.get_word_position(&word);
            self.lexem.push(Token {
                word,
                path: self.path,
                row,
                col
            });
            String::new()
        }
        else { word }
    }

    fn is_region(&self, is_matched: bool) -> bool {
        if let Some(region) = self.region.get_region() {
            region.preserve && !is_matched
        }
        else { false }
    }

    pub fn run(&mut self) {
        let mut word = String::new();
        while let Some(letter) = self.reader.next() {
            let is_matched = self.region.handle_region(&self.reader);
            // Handle region scope
            if self.is_region(is_matched) {
                word.push(letter);
            }
            else {
                // Skip whitespace
                if vec![' ', '\t'].contains(&letter) {
                    word = self.add_word(word);
                }
                // Handle special symbols
                else if self.symbols.contains(&letter) {
                    word = self.add_word(word);
                    word.push(letter);
                    word = self.add_word(word);
                    // TODO: When handling symbol, use peekable
                    // to handle the self.add_word(True)
                    // https://doc.rust-lang.org/stdx/iter/struct.Peekable.html
                }
                // Handle word
                else {
                    word.push(letter);
                }
            }
        }
        self.add_word(word);
    }
    // TODO: Handle lexing errors
}

#[cfg(test)]
mod test {
    #[test]
    fn test_lexer() {
        let symbols = vec!['(', ')'];
        let regions = vec![
            super::Region::new("string", "'", "'")
        ];
        let rules = super::Rules::new(symbols, regions);
        let mut cc = super::Compiler::new("TestScript", rules);
        cc.load("let a = (12 + 32)");
        
        let mut lexer = super::Lexer::new(&cc);
        lexer.run();
        println!("{:?}", lexer.lexem);
    }
}