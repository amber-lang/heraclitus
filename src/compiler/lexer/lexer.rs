use crate::compiler::{ Compiler, Token, SeparatorMode, ScopingMode };
use super::region_handler::{ RegionHandler, Reaction };
use super::reader::Reader;

// This is just an estimation of token amount
// inside of a typical 200-lined file.
const AVG_TOKEN_AMOUNT: usize = 1024;

pub struct Lexer<'a> {
    symbols: &'a Vec<char>,
    region: RegionHandler,
    reader: Reader<'a>,
    lexem: Vec<Token<'a>>,
    path: &'a String,
    separator_mode: SeparatorMode,
    scoping_mode: ScopingMode
}

impl<'a> Lexer<'a> {
    pub fn new<AST>(cc: &'a Compiler<AST>) -> Self {
        Lexer {
            symbols: &cc.rules.symbols,
            region: RegionHandler::new(&cc.rules),
            reader: Reader::new(&cc.code),
            lexem: Vec::with_capacity(AVG_TOKEN_AMOUNT),
            path: &cc.path,
            separator_mode: cc.separator_mode.clone(),
            scoping_mode: cc.scoping_mode.clone()
        }
    }

    // Add word that has been completed in previous iteration to the lexem
    fn add_word(&mut self, word: String) -> String {
        if word.len() > 0 {
            let (row, col) = self.reader.get_word_position(&word);
            self.lexem.push(Token {
                word,
                path: self.path,
                pos: (row, col)
            });
            String::new()
        }
        else { word }
    }

    // Add word that has been completed in current iteration to the lexem
    fn add_word_inclusively(&mut self, word: String) -> String {
        if word.len() > 0 {
            let (row, col) = self.reader.get_word_position(&word);
            self.lexem.push(Token {
                word,
                path: self.path,
                pos: (row, col + 1)
            });
            String::new()
        }
        else { word }
    }

    fn is_region(&self, reaction: Reaction) -> bool {
        if let Some(region) = self.region.get_region() {
            !region.tokenize && reaction == Reaction::Pass
        }
        else { false }
    }

    pub fn run(&mut self) {
        let mut word = String::new();
        while let Some(letter) = self.reader.next() {
            // Reaction stores the reaction of the region handler
            // Have we just opened or closed some region?
            let reaction = self.region.handle_region(&self.reader);
            match reaction {
                // If the region has been opened
                // Finish the part that we have been parsing
                Reaction::Open => {
                    word = self.add_word(word);
                    word.push(letter);
                },
                // If the region has been closed
                // Add the closing region and finish the word
                Reaction::Close => {
                    word.push(letter);
                    word = self.add_word_inclusively(word);
                }
                Reaction::Pass => {
                    // Handle region scope
                    if self.is_region(reaction) {
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
                            word = self.add_word_inclusively(word);
                        }
                        // Handle word
                        else {
                            word.push(letter);
                        }
                    }
                }
            }
        }
        self.add_word(word);
        self.region.handle_region_open(self.path, &self.reader);
    }
}

#[cfg(test)]
mod test {
    use crate::rules::{ Region, Rules };
    use crate::reg;
    use crate::compiler::{ Compiler };

    #[test]
    fn test_lexer_base() {
        let symbols = vec!['(', ')'];
        let regions = reg!([
            reg!(string as "String literal" => {
                begin: "'",
                end: "'"
            } in [
                reg!(array as "Array Literal" => {
                    begin: "[",
                    end: "]"
                })
            ])
        ]);
        let expected = vec![
            ("let".to_string(), 1, 1),
            ("a".to_string(), 1, 5),
            ("=".to_string(), 1, 7),
            ("(".to_string(), 1, 9),
            ("12".to_string(), 1, 10),
            ("+".to_string(), 1, 13),
            ("32".to_string(), 1, 15),
            (")".to_string(), 1, 17)
        ];
        type AST = ();
        let rules = Rules::new(symbols, regions);
        let mut cc: Compiler<AST> = Compiler::new("TestScript", rules);
        cc.load("let a = (12 + 32)");
        let mut lexer = super::Lexer::new(&cc);
        let mut result = vec![];
        // Simulate lexing
        lexer.run();
        for lex in lexer.lexem {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_lexer_string_interp() {
        let symbols = vec!['(', ')'];
        let regions = reg!([
            reg!(string_literal as "String literal" => {
                begin: "'",
                end: "'"
            } in [
                reg!(string_interp as "String interpolation" => {
                    begin: "{",
                    end: "}",
                    tokenize: true
                } ref global)
            ])
        ]);
        let expected = vec![
            ("let".to_string(), 1, 1),
            ("a".to_string(), 1, 5),
            ("=".to_string(), 1, 7),
            ("'this ".to_string(), 1, 9),
            ("{".to_string(), 1, 15),
            ("'is ".to_string(), 1, 16),
            ("{".to_string(), 1, 20),
            ("'reeeeaaaally'".to_string(), 1, 21),
            ("}".to_string(), 1, 35),
            (" long'".to_string(), 1, 36),
            ("}".to_string(), 1, 42),
            (" text'".to_string(), 1, 43)
        ];
        type AST = ();
        let rules = Rules::new(symbols, regions);
        let mut cc: Compiler<AST> = Compiler::new("TestScript", rules);
        cc.load("let a = 'this {'is {'reeeeaaaally'} long'} text'");
        let mut lexer = super::Lexer::new(&cc);
        let mut result = vec![];
        // Simulate lexing
        lexer.run();
        for lex in lexer.lexem {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }
}