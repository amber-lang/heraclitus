//! This is a logger module which is used by compiler to log errors, warnings and info messages

#![allow(dead_code)]
use colored::{Colorize, Color};
use pad::PadStr;
use crate::compiling::failing::position_info::PositionInfo;
use crate::compiling::failing::message::MessageType;
use crate::prelude::Position;

/// This is a logger that is used to log messages to the user
/// The logger is being used internally by the Message struct
/// when invoking the `show` method
pub struct Logger {
    kind: MessageType,
    trace: Vec<PositionInfo>
}

impl Logger {
    /// Create a new Displayer instance
    pub fn new(kind: MessageType, trace: &[PositionInfo]) -> Self {
        Logger {
            kind,
            trace: trace.to_vec()
        }
    }

    fn kind_to_color(&self) -> Color {
        match self.kind {
            MessageType::Error => Color::Red,
            MessageType::Warning => Color::Yellow,
            MessageType::Info => Color::Blue
        }
    }

    /// Render header of your information
    pub fn header(self, kind: MessageType) -> Self {
        let name = match kind {
            MessageType::Error => " ERROR ".to_string(),
            MessageType::Warning => " WARN ".to_string(),
            MessageType::Info => " INFO ".to_string()
        };
        let formatted = name
            .black()
            .bold()
            .on_color(self.kind_to_color());
        eprint!("{formatted} ");
        self
    }

    /// Render text with supplied coloring
    pub fn text(self, text: Option<String>) -> Self {
        if let Some(text) = text {
            eprint!("{}", text.color(self.kind_to_color()));
        }
        self
    }

    /// Render text with supplied coloring and end it with a newline
    pub fn line(self, text: Option<String>) -> Self {
        if let Some(text) = text {
            eprintln!("{}", text.color(self.kind_to_color()));
        }
        self
    }

    /// Render padded text with a newline, applying the supplied coloring, and end it with another newline
    pub fn padded_line(self, text: Option<String>) -> Self {
        if let Some(text) = text {
            eprintln!("\n{}", text.color(self.kind_to_color()));
        }
        self
    }

    /// Render location details with supplied coloring
    pub fn path(self) -> Self {
        let get_row_col = |pos: &PositionInfo| match pos.position {
            Position::Pos(row, col) => format!("{}:{}", row, col),
            Position::EOF => " end of file".to_string()
        };
        let path = match self.trace.first() {
            Some(pos) => {
                [
                    format!("at {}:{}", pos.get_path(), get_row_col(pos)),
                    self.trace.iter()
                        .skip(1)
                        .map(|pos| format!("in {}:{}", pos.get_path(), get_row_col(pos)))
                        .collect::<Vec<String>>()
                        .join("\n")
                ].join("\n")
            },
            None => {
                "at [unknown]:0:0".to_string()
            }
        }.trim_end().to_string();
        eprintln!("{}", path.color(self.kind_to_color()).dimmed());
        self
    }

    // Returns last row, column and it's length
    fn get_row_col_len(&self) -> Option<(usize, usize, usize)> {
        match self.trace.first() {
            Some(pos) => match pos.position {
                Position::Pos(row, col) => Some((row, col, pos.len)),
                Position::EOF => None
            },
            None => None
        }
    }

    // Get max padding size (for line numbering)
    fn get_max_pad_size(&self, length: usize) -> Option<usize> {
        let (row, _, _) = self.get_row_col_len()?;
        if row < length - 1 {
            Some(format!("{}", row + 1).len())
        }
        else {
            Some(format!("{}", row).len())
        }
    }

    // Returns chopped string where fisrt and third part are supposed
    // to be left as is but the second one is supposed to be highlighted
    fn get_highlighted_part(&self, line: &str) -> Option<[String;3]> {
        let (_row, col, len) = self.get_row_col_len()?;
        let begin = col - 1;
        let end = begin + len;
        let mut results: [String; 3] = Default::default();
        for (index, letter) in line.chars().enumerate() {
            if index < begin {
                results[0].push(letter);
            }
            else if index >= end {
                results[2].push(letter);
            }
            else {
                results[1].push(letter);
            }
        }
        Some(results)
    }

    // Return requested row with appropriate coloring
    fn get_snippet_row(&self, code: &Vec<String>, index: usize, offset: i8, overflow: &mut usize) -> Option<String> {
        let (row, col, len) = self.get_row_col_len()?;
        let max_pad = self.get_max_pad_size(code.len())?;
        let index = index as i32 + offset as i32;
        let row = row as i32 + offset as i32;
        let code = code.get(index as usize)?.clone();
        let line = format!("{row}").pad_to_width(max_pad);
        // Case if we are in the same line as the error (or message)
        if offset == 0 {
            let slices = self.get_highlighted_part(&code)?;
            let formatted = format!("{}{}{}", slices[0], slices[1].color(self.kind_to_color()), slices[2]);
            // If we are at the end of the code snippet and there is still some
            if col + len - 1 > code.chars().count() {
                // We substract here 2 because 1 is the offset of col (starts at 1)
                // and other 1 is the new line character that we do not display
                *overflow = (col + len - 2).checked_sub(code.chars().count()).unwrap_or(0);
            }
            Some(format!("{line}| {formatted}"))
        }
        // Case if we are in a different line than the error (or message)
        else {
            // If there is some overflow value - display it as well
            if *overflow > 0 {
                // Case if all line is highlighted
                if *overflow > code.chars().count() {
                    Some(format!("{line}| {}", code.color(self.kind_to_color())).dimmed().to_string())
                }
                // Case if some line is highlighted
                else {
                    let err = code.get(0..*overflow).unwrap().to_string().color(self.kind_to_color());
                    let rest = code.get(*overflow..).unwrap().to_string();
                    Some(format!("{line}| {err}{rest}").dimmed().to_string())
                }
            }
            // Case if no overflow
            else {
                Some(format!("{line}| {code}").dimmed().to_string())
            }
        }
    }

    /// Render snippet of the code if the message is contextual to it
    pub fn snippet<T: AsRef<str>>(self, code: Option<T>) -> Self {
        if let Some(pos) = self.trace.first() {
            if let Ok(code) = std::fs::read_to_string(pos.get_path()) {
                self.snippet_from_code(code);
                return self;
            }
        }
        if let Some(code) = code {
            self.snippet_from_code(code.as_ref().to_string());
        }
        self
    }

    /// Render snippet of the code based on the code data
    fn snippet_from_code(&self, code: String) -> Option<()> {
        let (row, _, _) = self.get_row_col_len()?;
        let mut overflow = 0;
        let index = row - 1;
        let code = code.split('\n')
            .map(|item| item.trim_end().to_string())
            .collect::<Vec<String>>();
        eprintln!();
        // Show additional code above the snippet
        if let Some(line) = self.get_snippet_row(&code, index, -1, &mut overflow) {
            eprintln!("{}", line);
        }
        // Show the current line of code
        eprintln!("{}", self.get_snippet_row(&code, index, 0, &mut overflow)?);
        // Show additional code below the snippet
        eprintln!("{}", self.get_snippet_row(&code, index, 1, &mut overflow)?);
        Some(())
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use std::time::Duration;
    use std::thread::sleep;

    use crate::prelude::{DefaultMetadata, Metadata, MessageType, PositionInfo, Token};
    #[allow(unused_variables)]

    #[test]
    fn test_displayer() {
        let code = vec![
            "let a = 12",
            "value = 'this",
            "is mutltiline",
            "code'"
        ].join("\n");
        // Uncomment to see the error message
        sleep(Duration::from_secs(1));
        let trace = [
            PositionInfo::at_pos(Some("/path/to/bar".to_string()), (3, 4), 10),
            PositionInfo::at_pos(Some("/path/to/foo".to_string()), (2, 9), 24),
        ];
        super::Logger::new(MessageType::Error, &trace)
            .header(MessageType::Error)
            .line(Some(format!("Cannot call function \"foobar\" on a number")))
            .path()
            .snippet(Some(code));
    }

    #[test]
    fn test_end_of_line_displayer() {
        let code = vec![
            "hello"
        ].join("\n");
        // Uncomment to see the error message
        sleep(Duration::from_secs(1));
        let trace = [
            PositionInfo::at_pos(Some("/path/to/foo".to_string()), (2, 6), 1)
        ];
        super::Logger::new(MessageType::Error, &trace)
            .header(MessageType::Error)
            .line(Some(format!("Cannot call function \"foobar\" on a number")))
            .path()
            .snippet(Some(code));
    }

    #[test]
    fn test_between_tokens() {
        let code = vec![
            "foo(12 + 24)"
        ].join("\n");
        // Uncomment to see the error message
        sleep(Duration::from_secs(1));
        let begin = Token { word: "12".to_string(), pos: (1, 5), start: 4 };
        let end = Token { word: ")".to_string(), pos: (1, 12), start: 11 };
        let mut meta = DefaultMetadata::new(vec![], Some("/path/to/foo".to_string()), Some(code.clone()));
        let trace = [
            PositionInfo::from_between_tokens(&mut meta, Some(begin), Some(end))
        ];
        super::Logger::new(MessageType::Error, &trace)
            .header(MessageType::Error)
            .line(Some(format!("Cannot call function \"foobar\" on a number")))
            .path()
            .snippet(Some(code));
    }
}
