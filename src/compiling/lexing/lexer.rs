//!  Lexer
//!
//! This module contains the  lexer that is used to tokenize the source code

use crate::{
    compiling_rules::Rules,
    prelude::{PositionInfo, ScopingMode, SeparatorMode, Token},
};

use super::{
    compound_handler::{CompoundHandler, CompoundReaction},
    reader::Reader,
    region_handler::{RegionHandler, RegionReaction},
    LexerError, LexerErrorType,
};

///  Lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    rules: Rules,
    /// Path to the lexed file
    pub path: Option<String>,
    /// Separator mode for this lexer
    pub separator_mode: SeparatorMode,
    /// Escape symbol for this lexer. Default is '\\'
    pub escape_symbol: char,
    /// Scoping mode for this lexer
    pub scoping_mode: ScopingMode,
}

struct LexState {
    word: String,
    is_indenting: bool,
    is_escaped: bool,
    token_start_index: usize,
    position: (usize, usize),
    reader: Reader,
    lexem: Vec<Token>,
    region_handler: RegionHandler,
    compound_handler: CompoundHandler,
}

impl Lexer {
    /// Create a new Lexer based on the compiler metadata
    pub fn new(rules: Rules) -> Self {
        Lexer {
            rules,
            path: None,
            separator_mode: SeparatorMode::Manual,
            escape_symbol: '\\',
            scoping_mode: ScopingMode::Block,
        }
    }

    /// Add indentation to the lexem
    #[inline]
    fn add_indent(&self, lex_state: &mut LexState) {
        if lex_state.word.is_empty() {
            return;
        }

        // Getting position by word here would attempt to
        // substract with overflow since the new line character
        // technically belongs to the previous line
        let (row, _col) = lex_state.reader.get_position();
        lex_state.lexem.push(Token {
            word: lex_state.word.clone(),
            pos: (row, 1),
            start: lex_state.token_start_index,
        });
        lex_state.position = (0, 0);
        lex_state.word = String::new();
    }

    /// Add word that has been completed in previous iteration to the lexem
    #[inline]
    fn add_word(&self, lex_state: &mut LexState) {
        if lex_state.word.is_empty() {
            return;
        }

        lex_state.lexem.push(Token {
            word: lex_state.word.clone(),
            pos: lex_state.position,
            start: lex_state.token_start_index,
        });
        lex_state.position = (0, 0);
        lex_state.word = String::new();
    }

    /// Add word that has been completed in current iteration to the lexem
    #[inline]
    fn add_word_inclusively(&self, lex_state: &mut LexState) {
        if lex_state.word.is_empty() {
            return;
        }

        lex_state.lexem.push(Token {
            word: lex_state.word.clone(),
            pos: lex_state.position,
            start: lex_state.token_start_index,
        });
        lex_state.position = (0, 0);
        lex_state.word = String::new()
    }

    /// Checks whether this is a nontokenizable region
    #[inline]
    fn is_tokenized_region(&self, reaction: &RegionReaction, lex_state: &mut LexState) -> bool {
        if let Some(region) = lex_state.region_handler.get_region() {
            region.tokenize && *reaction == RegionReaction::Pass
        } else {
            false
        }
    }

    /// Pattern code for adding a symbol
    /// **[*]**
    #[inline]
    fn pattern_add_symbol(&self, lex_state: &mut LexState, letter: char) {
        self.add_word(lex_state);

        if lex_state.word.is_empty() {
            lex_state.token_start_index = lex_state.reader.get_index();
        }
        self.word_push(lex_state, letter);
        lex_state.position = lex_state.reader.get_position();

        self.add_word_inclusively(lex_state);
    }

    /// Pattern code for beginning a new region
    /// **[**
    #[inline]
    fn pattern_begin(&self, lex_state: &mut LexState, letter: char) {
        self.add_word(lex_state);
        self.word_push(lex_state, letter);
    }

    /// Pattern code for ending current region
    /// **]**
    #[inline]
    fn pattern_end(&self, lex_state: &mut LexState, letter: char) {
        self.word_push(lex_state, letter);
        self.add_word_inclusively(lex_state);
    }

    /// Push letter to the word and set token start index
    fn word_push(&self, lex_state: &mut LexState, letter: char) {
        if lex_state.word.is_empty() {
            lex_state.token_start_index = lex_state.reader.get_index();
        }
        lex_state.word.push(letter);
    }

    /// Tokenize source code
    ///
    /// Run lexer and tokenize code. The result is stored in the lexem attribute
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, LexerError> {
        let code = input.to_string();

        let mut lex_state = LexState {
            word: String::new(),
            is_indenting: false,
            is_escaped: false,
            token_start_index: 0,
            position: (0, 0),
            lexem: Vec::new(),
            reader: Reader::new(&code),
            region_handler: RegionHandler::new(&self.rules),
            compound_handler: CompoundHandler::new(&self.rules),
        };

        while let Some(letter) = lex_state.reader.next() {
            /****************/
            /* Set Position */
            /****************/

            // If the new position hasn't been set yet, set it
            if lex_state.position == (0, 0) {
                // If separator mode is set to Manual and the letter is a separator,
                // then skip finding a new position
                if SeparatorMode::Manual != self.separator_mode || letter != '\n' {
                    let region = lex_state.region_handler.get_region().unwrap();
                    // If the region is tokenized, then check if the letter is a separator
                    if !region.tokenize || !vec![' ', '\t'].contains(&letter) {
                        lex_state.position = lex_state.reader.get_position();
                    }
                }
            }

            // Reaction stores the reaction of the region handler
            // Have we just opened or closed some region?
            let reaction = lex_state
                .region_handler
                .handle_region(&lex_state.reader, lex_state.is_escaped);
            match reaction {
                // If the region has been opened
                // Finish the part that we have been parsing
                RegionReaction::Begin(tokenize) => {
                    // Also if the new region is an interpolation that tokenizes
                    // the inner content - separate the region from the content
                    if tokenize {
                        self.pattern_add_symbol(&mut lex_state, letter);
                    }
                    // Regular region case
                    else {
                        // This is supposed to prevent overshadowing new line
                        // character if region rule opens with newline
                        if letter == '\n' {
                            // This additionally creates a new token
                            self.pattern_add_symbol(&mut lex_state, letter);
                        }
                        // Normally start a new region
                        self.pattern_begin(&mut lex_state, letter);
                    }
                }
                // If the region has been closed
                // Add the closing region and finish the word
                RegionReaction::End(tokenize) => {
                    // Also if the new region is an interpolation that tokenizes
                    // the inner content - separate the region from the content
                    if tokenize {
                        self.pattern_add_symbol(&mut lex_state, letter);
                    }
                    // Regular region case
                    else {
                        // Normally close the region
                        self.pattern_end(&mut lex_state, letter);
                        // This is supposed to prevent overshadowing new line
                        // character if region rule closes with newline
                        if letter == '\n' {
                            // This additionally creates a new token
                            self.pattern_add_symbol(&mut lex_state, letter);
                        }
                    }
                }
                RegionReaction::Pass => {
                    let is_tokenized_region = self.is_tokenized_region(&reaction, &mut lex_state);
                    match lex_state.compound_handler.handle_compound(
                        letter,
                        &lex_state.reader,
                        is_tokenized_region,
                    ) {
                        CompoundReaction::Begin => self.pattern_begin(&mut lex_state, letter),
                        CompoundReaction::Keep => self.word_push(&mut lex_state, letter),
                        CompoundReaction::End => self.pattern_end(&mut lex_state, letter),
                        CompoundReaction::Pass => {
                            // Handle region scope
                            if !self.is_tokenized_region(&reaction, &mut lex_state) {
                                let region = lex_state.region_handler.get_region().unwrap();
                                // Flip escaped key
                                lex_state.is_escaped = (!lex_state.is_escaped
                                    && letter == self.escape_symbol)
                                    .then(|| !lex_state.is_escaped)
                                    .unwrap_or(false);
                                // Handle singleline attribute
                                if letter == '\n' && region.singleline {
                                    let pos = lex_state.reader.get_position();
                                    return Err((
                                        LexerErrorType::Singleline,
                                        PositionInfo::at_pos(self.path.clone(), pos, 0)
                                            .data(region.name.clone()),
                                    ));
                                }
                                self.word_push(&mut lex_state, letter);
                            } else {
                                /******************/
                                /* Mode modifiers */
                                /******************/

                                // Create indent regions: '\n   '
                                if let ScopingMode::Indent = self.scoping_mode {
                                    // If we are still in the indent region - proceed
                                    if lex_state.is_indenting && vec![' ', '\t'].contains(&letter) {
                                        self.word_push(&mut lex_state, letter);
                                    }
                                    // If it's the new line - start indent region
                                    if letter == '\n' {
                                        lex_state.is_indenting = true;
                                        self.pattern_begin(&mut lex_state, letter);
                                    }
                                    // Check if the current letter
                                    // concludes current indent region
                                    if lex_state.is_indenting {
                                        if let Some(next_char) = lex_state.reader.peek() {
                                            if !vec![' ', '\t'].contains(&next_char) {
                                                self.add_indent(&mut lex_state);
                                                lex_state.is_indenting = false;
                                            }
                                        }
                                        continue;
                                    }
                                }
                                // Skip newline character if we want to manually insert semicolons
                                if let SeparatorMode::Manual = self.separator_mode {
                                    if letter == '\n' {
                                        self.add_word(&mut lex_state);
                                        continue;
                                    }
                                }

                                /*****************/
                                /* Regular Lexer */
                                /*****************/

                                // Skip whitespace
                                if vec![' ', '\t'].contains(&letter) {
                                    self.add_word(&mut lex_state);
                                }
                                // Handle special symbols
                                else if self.rules.symbols.contains(&letter) || letter == '\n' {
                                    self.pattern_add_symbol(&mut lex_state, letter);
                                }
                                // Handle word
                                else {
                                    self.word_push(&mut lex_state, letter);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.add_word(&mut lex_state);
        // If some region exists that was not closed
        if let Err((pos, region)) = lex_state.region_handler.is_region_closed(&lex_state.reader) {
            return Err((
                LexerErrorType::Unclosed,
                PositionInfo::at_pos(self.path.clone(), pos, 0).data(region.name),
            ));
        }

        Ok(lex_state.lexem)
    }
}

#[cfg(test)]
mod test {
    use crate::compiling::ScopingMode;
    use crate::compiling_rules::{Region, Rules};
    use crate::reg;

    #[test]
    fn test_lexer_base() {
        let symbols = vec!['(', ')'];
        let regions = reg![reg!(string as "String literal" => {
            begin: "'",
            end: "'"
        } => [
            reg!(array as "Array Literal" => {
                begin: "[",
                end: "]"
            })
        ])];
        let expected = vec![
            ("let".to_string(), 1, 1),
            ("a".to_string(), 1, 5),
            ("=".to_string(), 1, 7),
            ("(".to_string(), 1, 9),
            ("12".to_string(), 1, 10),
            ("+".to_string(), 1, 13),
            ("32".to_string(), 1, 15),
            (")".to_string(), 1, 17),
        ];
        let rules = Rules::new(symbols, vec![], regions);
        let lexer = super::Lexer::new(rules);
        let mut result = vec![];
        // Simulate lexing
        let res = lexer.tokenize("let a = (12 + 32)");
        assert!(res.is_ok());
        for lex in res.unwrap() {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_lexer_string_interp() {
        let symbols = vec!['(', ')'];
        let regions = reg![reg!(string_literal as "String literal" => {
            begin: "'",
            end: "'"
        } => [
            reg!(string_interp as "String interpolation" => {
                begin: "{",
                end: "}",
                tokenize: true
            } ref global)
        ])];
        let expected = vec![
            ("let".to_string(), 1, 1),
            ("a".to_string(), 1, 5),
            ("=".to_string(), 1, 7),
            ("'this ".to_string(), 1, 9),
            ("{".to_string(), 1, 15),
            ("'is ".to_string(), 1, 16),
            ("{".to_string(), 1, 20),
            ("adjective".to_string(), 1, 21),
            ("}".to_string(), 1, 30),
            (" long'".to_string(), 1, 31),
            ("}".to_string(), 1, 37),
            (" 🎉 text'".to_string(), 1, 38),
        ];
        let rules = Rules::new(symbols, vec![], regions);

        let lexer = super::Lexer::new(rules);
        let mut result = vec![];
        // Simulate lexing
        let res = lexer.tokenize("let a = 'this {'is {adjective} long'} 🎉 text'");
        assert!(res.is_ok());
        for lex in res.unwrap() {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_lexer_indent_scoping_mode() {
        let symbols = vec![':'];
        let regions = reg![];
        let expected = vec![
            ("if".to_string(), (1, 1), 0),
            ("condition".to_string(), (1, 4), 3),
            (":".to_string(), (1, 13), 12),
            ("\n    ".to_string(), (2, 1), 13),
            ("if".to_string(), (2, 5), 18),
            ("subcondition".to_string(), (2, 8), 21),
            (":".to_string(), (2, 20), 33),
            ("\n        ".to_string(), (3, 1), 34),
            ("pass".to_string(), (3, 9), 43),
        ];
        let rules = Rules::new(symbols, vec![], regions);

        let mut lexer = super::Lexer::new(rules);
        lexer.scoping_mode = ScopingMode::Indent;
        let mut result = vec![];
        // Simulate lexing
        let res = lexer
            .tokenize(&vec!["if condition:", "    if subcondition:", "        pass"].join("\n"));
        assert!(res.is_ok());
        for lex in res.unwrap() {
            result.push((lex.word, (lex.pos.0, lex.pos.1), lex.start));
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
            (";".to_string(), 3, 3),
        ];
        let rules = Rules::new(symbols, vec![], regions);
        let lexer = super::Lexer::new(rules);
        let mut result = vec![];
        // Simulate lexing
        let res = lexer.tokenize(&vec!["let age = 12", "+", "12;"].join("\n"));
        assert!(res.is_ok());
        for lex in res.unwrap() {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_lexer_multiline_regions() {
        let symbols = vec![';', '+', '='];
        let regions = reg![reg!(string as "String" => {
            begin: "'",
            end: "'"
        })];
        let expected = vec![("'this\nis\na\nmultiline\nstring'".to_string(), 1, 1)];
        let rules = Rules::new(symbols, vec![], regions);
        let lexer = super::Lexer::new(rules);
        let mut result = vec![];
        // Simulate lexing
        let res = lexer.tokenize(&vec!["'this", "is", "a", "multiline", "string'"].join("\n"));
        assert!(res.is_ok());
        for lex in res.unwrap() {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_lexer_escaped_regions() {
        let symbols = vec![';', '+', '='];
        let regions = reg![reg!(string as "String" => {
            begin: "\"",
            end: "\""
        })];
        let expected = vec![("\"this is \\\"escaped\\\" string\"".to_string(), 1, 1)];
        let rules = Rules::new(symbols, vec![], regions);
        let lexer = super::Lexer::new(rules);
        let mut result = vec![];
        // Simulate lexing
        let res = lexer.tokenize(&vec!["\"this is \\\"escaped\\\" string\""].join("\n"));
        assert!(res.is_ok());
        for lex in res.unwrap() {
            result.push((lex.word, lex.pos.0, lex.pos.1));
        }
        assert_eq!(expected, result);
    }
}
