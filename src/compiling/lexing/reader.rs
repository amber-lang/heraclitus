#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

const BEGINNING: (usize, usize) = (0, 1);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReadMode {
    History,
    Future
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Reader {
    pub code: String,
    pub row: usize,
    pub col: usize,
    pub index: usize,
    pub new_line: bool
}

impl Reader {
    pub fn new(code: &String) -> Self {
        Reader {
            code: code.clone(),
            row: BEGINNING.0,
            col: BEGINNING.1,
            index: 0,
            new_line: true
        }
    }

    #[inline]
    pub fn next_letter(&mut self) -> Option<char> {
        if self.row > 0 {
            self.index += 1;
            self.col += 1;
        }
        if self.new_line {
            self.new_line = false;
            self.row += 1;
            self.col = BEGINNING.1;
        }
        match self.code.chars().nth(self.index) {
            Some(letter) => {
                self.new_line = letter == '\n';
                Some(letter)
            }
            None => None
        }
    }

    /// Return current index of the string
    #[inline]
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Return current position in code
    #[inline]
    pub fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    /// Gets position of token that has been read
    #[inline]
    pub fn get_word_position(&self, word: &str) -> (usize, usize) {
        (self.row, self.col - word.chars().count())
    }

    /// Workaround for UTF-8 symbols
    #[inline]
    fn get_slice(&self, begin: usize, end: usize) -> String {
        self.code.chars().skip(begin).take(end - begin).collect::<String>()
    }

    /// Get last n characters that were processed in correct order
    /// This function includes currently processed character
    #[inline]
    pub fn get_history(&self, n: usize) -> Option<String> {
        let offset = self.index + 1;
        // Handle arithmetic overflow
        let begin = if offset >= n { offset - n } else { return None };
        let end = offset;
        Some(self.get_slice(begin, end))
    }

    /// Show next character that is going to be consumed
    #[inline]
    pub fn peek(&self) -> Option<char> {
        // Amount required to peek one item in the future
        let one_forward = 2;
        match self.get_future(one_forward) {
            Some(value) => value.chars().nth(one_forward - 1),
            None => None
        }
    }

    /// Show next character that is going to be consumed depending on the mode
    #[inline]
    pub fn get_history_or_future(&self, n: usize, mode: &ReadMode) -> Option<String> {
        match mode {
            ReadMode::History => self.get_history(n),
            ReadMode::Future => self.get_future(n)
        }
    }

    /// Get next n characters that will be processed in correct order
    /// This function includes currently processed character
    #[inline]
    pub fn get_future(&self, n: usize) -> Option<String> {
        let begin = self.index;
        // Handle arithmetic overflow
        let end = if self.index + n <= self.code.len() { self.index + n } else { return None };
        // Find the next UTF-8 character boundary for the beginning of the slice
        Some(self.get_slice(begin, end))
    }
}

impl Iterator for Reader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_letter()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn letter_position() {
        let code = vec![
            "apple",
            "banana",
            "orange"
        ].join("\n");
        let expected = vec![
            // a      p       p       l       e       \n
            (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
            // b      a       n       a       n       a       \n
            (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
            // o      r       a       n       g       e
            (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6)
        ];
        let mut reader = super::Reader::new(&code);
        let mut result = vec![];
        // Simulate lexing
        while let Some(_) = reader.next() {
            let pos = reader.get_position();
            result.push(pos);
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn index_position() {
        let code = vec![
            "apple",
            "orange",
        ].join("\n");
        let expected = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
        ];
        let mut reader = super::Reader::new(&code);
        let mut result = vec![];
        // Simulate lexing
        while let Some(_) = reader.next() {
            let pos = reader.get_index();
            result.push(pos);
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn correct_history_and_future() {
        const SIZE: usize = 5;
        let code = vec![
            "apple",
            "kiwi"
        ].join("\n");
        let expected = vec![
            "apple",
            "pple\n",
            "ple\nk",
            "le\nki",
            "e\nkiw",
            "\nkiwi"
        ];
        let mut reader = super::Reader::new(&code);
        let mut result_history = vec![];
        let mut result_future = vec![];
        // Simulate lexing
        while let Some(_) = reader.next() {
            if let Some(history) = reader.get_history(SIZE) {
                result_history.push(history.clone());
            }
            if let Some(future) = reader.get_future(SIZE) {
                result_future.push(future.clone());
            }
        }
        assert_eq!(expected, result_history);
        assert_eq!(expected, result_future);
    }
}
