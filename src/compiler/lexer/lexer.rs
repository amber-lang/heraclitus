use crate::Region;
use crate::compiler::{ Compiler, Token, SeparatorMode, ScopingMode };
use super::region_handler::{ RegionHandler, Reaction };
use super::reader::Reader;
use crate::compiler::logger::{ Log, LogMessage };

// This is just an estimation of token amount
// inside of a typical 200-lined file.
const AVG_TOKEN_AMOUNT: usize = 1024;

pub enum LexerError {
    // Unspillable region has been spilled
    Singleline,
    // Given region left unclosed
    Unclosed
}

pub type LexerMessage = (LexerError, LogMessage);

pub struct Lexer<'a> {
    symbols: &'a Vec<char>,
    region: RegionHandler,
    reader: Reader<'a>,
    pub lexem: Vec<Token>,
    path: Option<String>,
    separator_mode: SeparatorMode,
    scoping_mode: ScopingMode
}

impl<'a> Lexer<'a> {
    pub fn new(cc: &'a Compiler) -> Self {
        Lexer {
            symbols: &cc.rules.symbols,
            region: RegionHandler::new(&cc.rules),
            reader: Reader::new(&cc.code),
            lexem: Vec::with_capacity(AVG_TOKEN_AMOUNT),
            path: cc.path.clone(),
            separator_mode: cc.separator_mode.clone(),
            scoping_mode: cc.scoping_mode.clone()
        }
    }

    /// Add indentation to the lexem
    fn add_indent(&mut self, word: String) -> String {
        if word.len() > 0 {
            // Getting position by word here would attempt to
            // substract with overflow since the new line character
            // technically belongs to the previous line
            let (row, _col) = self.reader.get_position();
            self.lexem.push(Token {
                word,
                pos: (row, 1)
            });
            String::new()
        } else { word }
    }

    /// Add word that has been completed in previous iteration to the lexem
    fn add_word(&mut self, word: String) -> String {
        if word.len() > 0 {
            let (row, col) = self.reader.get_word_position(&word);
            self.lexem.push(Token {
                word,
                pos: (row, col)
            });
            String::new()
        }
        else { word }
    }

    /// Add word that has been completed in current iteration to the lexem
    fn add_word_inclusively(&mut self, word: String) -> String {
        if word.len() > 0 {
            let (row, col) = self.reader.get_word_position(&word);
            self.lexem.push(Token {
                word,
                pos: (row, col + 1)
            });
            String::new()
        }
        else { word }
    }

    /// Checks whether this is a nontokenizable region
    fn is_non_token_region(&self, reaction: Reaction) -> bool {
        if let Some(region) = self.region.get_region() {
            !region.tokenize && reaction == Reaction::Pass
        }
        else { false }
    }

    /// Pattern code for adding a symbol
    /// **[*]**
    fn pattern_add_symbol(&mut self, mut word: String, letter: char) -> String {
        word = self.add_word(word);
        word.push(letter);
        self.add_word_inclusively(word)
    }

    /// Pattern code for beginning a new region
    /// **[**
    fn pattern_begin_region(&mut self, mut word: String, letter: char) -> String {
        word = self.add_word(word);
        word.push(letter);
        word
    }

    /// Pattern code for ending current region
    /// **]**
    fn pattern_end_region(&mut self, mut word: String, letter: char) -> String {
        word.push(letter);
        self.add_word_inclusively(word)
    }

    pub fn run(&mut self) -> Result<(),LexerMessage> {
        let mut word = String::new();
        let mut is_indenting = false;
        while let Some(letter) = self.reader.next() {
            // Reaction stores the reaction of the region handler
            // Have we just opened or closed some region?
            let reaction = self.region.handle_region(&self.reader);
            match reaction {
                // If the region has been opened
                // Finish the part that we have been parsing
                Reaction::Begin => {
                    // This is supposed to prevent overshadowing new line
                    // character if region rule opens with newline
                    if letter == '\n' {
                        word = self.pattern_add_symbol(word, letter);
                    }
                    word = self.pattern_begin_region(word, letter);
                },
                // If the region has been closed
                // Add the closing region and finish the word
                Reaction::End => {
                    word = self.pattern_end_region(word, letter);
                    // This is supposed to prevent overshadowing new line
                    // character if region rule closes with newline
                    if letter == '\n' {
                        word = self.pattern_add_symbol(word, letter);
                    }
                }
                Reaction::Pass => {
                    // Handle region scope
                    if self.is_non_token_region(reaction) {
                        let region = self.region.get_region().unwrap();
                        // Handle singleline attribute
                        if letter == '\n' && region.singleline {
                            let (row, col) = self.reader.get_position();
                            return Err((
                                LexerError::Singleline,
                                LogMessage::new(self.path.clone(), row, col)
                                    .attach_code(self.reader.code.clone())
                                    .attach_metadata(region.name.clone())
                            ))
                        }
                        word.push(letter);
                    }
                    else {

                        /******************/
                        /* Mode modifiers */
                        /******************/

                        // Create indent regions: '\n   '
                        if let ScopingMode::Indent = self.scoping_mode {
                            // If we are still in the indent region - proceed
                            if is_indenting && vec![' ', '\t'].contains(&letter) {
                                word.push(letter);
                            }
                            // If it's the new line - start indent region
                            if letter == '\n' {
                                is_indenting = true;
                                word = self.pattern_begin_region(word, letter);
                            }
                            // Check if the current letter
                            // concludes current indent region
                            if is_indenting {
                                if let Some(next_char) = self.reader.peek() {
                                    if !vec![' ', '\t'].contains(&next_char) {
                                        word = self.add_indent(word);
                                        is_indenting = false;
                                    }
                                }
                                continue
                            }
                        }
                        // Skip newline character if we want to manually insert semicolons
                        if let SeparatorMode::Manual = self.separator_mode {
                            if letter == '\n' {
                                word = self.add_word(word);
                                continue
                            }
                        }

                        /*****************/
                        /* Regular Lexer */
                        /*****************/

                        // Skip whitespace
                        if vec![' ', '\t'].contains(&letter) {
                            word = self.add_word(word);
                        }
                        // Handle special symbols
                        else if self.symbols.contains(&letter) || letter == '\n' {
                            word = self.pattern_add_symbol(word, letter);
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
        if let Err((row, col, region)) = self.region.is_region_closed(&self.reader) {
            return Err((
                LexerError::Unclosed,
                LogMessage::new(self.path.clone(), row, col)
                    .attach_code(self.reader.code.clone())
                    .attach_metadata(region.name)
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::rules::{ Region, Rules };
    use crate::reg;
    use crate::compiler::{ Compiler, ScopingMode, SeparatorMode };

    #[test]
    fn test_lexer_base() {
        let symbols = vec!['(', ')'];
        let regions = reg![
            reg!(string as "String literal" => {
                begin: "'",
                end: "'"
            } => [
                reg!(array as "Array Literal" => {
                    begin: "[",
                    end: "]"
                })
            ])
        ];
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
        let rules = Rules::new(symbols, regions);
        let mut cc: Compiler = Compiler::new("TestScript", rules);
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
        let regions = reg![
            reg!(string_literal as "String literal" => {
                begin: "'",
                end: "'"
            } => [
                reg!(string_interp as "String interpolation" => {
                    begin: "{",
                    end: "}",
                    tokenize: true
                } ref global)
            ])
        ];
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
        let rules = Rules::new(symbols, regions);
        let mut cc: Compiler = Compiler::new("TestScript", rules);
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

    #[test]
    fn test_lexer_indent_scoping_mode() {
        let symbols = vec![':'];
        let regions = reg![];
        let expected = vec![
            ("if".to_string(), 1, 1),
            ("condition".to_string(), 1, 4),
            (":".to_string(), 1, 13),
            ("\n    ".to_string(), 2, 1),
            ("if".to_string(), 2, 5),
            ("subcondition".to_string(), 2, 8),
            (":".to_string(), 2, 20),
            ("\n        ".to_string(), 3, 1),
            ("pass".to_string(), 3, 9)
        ];
        let rules = Rules::new(symbols, regions);
        let mut cc: Compiler = Compiler::new("Testhon", rules);
        cc.scoping_mode = ScopingMode::Indent;
        cc.load(vec![
            "if condition:",
            "    if subcondition:",
            "        pass"
        ].join("\n"));
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
    fn test_lexer_manual_separator_mode() {
        let symbols = vec![';', '+', '='];
        let regions = reg![];
        let expected = vec![
            ("let".to_string(), 1, 1),
            ("age".to_string(), 1, 5),
            ("=".to_string(), 1, 9),
            ("12".to_string(), 1, 11),
            ("+".to_string(), 2, 1),
            ("12".to_string(), 3, 1),
            (";".to_string(), 3, 3)
        ];
        let rules = Rules::new(symbols, regions);
        let mut cc: Compiler = Compiler::new("Testhon", rules);
        cc.separator_mode = SeparatorMode::Manual;
        cc.load(vec![
            "let age = 12",
            "+",
            "12;"
        ].join("\n"));
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