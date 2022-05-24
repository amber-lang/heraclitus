use crate::compiler::token::Token;

const BEGINNING: (usize, usize) = (1, 1);

pub struct Reader<'a> {
    pub code: &'a String,
    pub row: usize,
    pub col: usize,
    pub index: usize
}

impl<'a> Reader<'a> {
    pub fn new(code: &'a String) -> Self {
        Reader {
            code,
            row: BEGINNING.0,
            col: BEGINNING.1,
            index: 0
        }
    }

    pub fn next_letter(&mut self) -> Option<char> {
        match self.code.chars().nth(self.index) {
            Some(letter) => {
                self.index += 1;
                self.col += 1;
                if letter == '\n' {
                    self.row += 1;
                    self.col = BEGINNING.1;
                }
                Some(letter)
            }
            None => None
        }
    }

    // Gets position of token that has been read
    pub fn get_token_position(&self, token: Token) -> (usize, usize) {
        (self.row, self.col - token.word.len())
    }

    // Get last n characters that were processed in correct order
    pub fn get_history(&self, n: usize) -> Option<&str> {
        // Handle arithmetic overflow
        let begin = if self.index >= n { self.index - n } else { return None };
        let end = self.index;
        Some(&self.code[begin..end])
    }

    // Get next n characters that will be processed in correct order
    pub fn get_future(&self, n: usize) -> Option<&str> {
        let begin = self.index;
        // Handle arithmetic overflow
        let end = if self.index + n < self.code.len() { self.index + n } else { return None };
        Some(&self.code[begin..end])
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_letter()
    }
}