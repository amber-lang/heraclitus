//! This is a logger module which is used by compiler to log errors, warnings and info messages

use colored::{Colorize, Color};
use pad::PadStr;
use crate::compiling::failing::{message::Snippet, position_info::PositionInfo};
use crate::compiling::failing::message::MessageType;
use crate::prelude::Position;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// This is a logger that is used to log messages to the user
/// The logger is being used internally by the Message struct
/// when invoking the `show` method
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    // Get padding size for line numbering (width of the largest line number shown)
    fn get_pad_size(snippet: &Snippet) -> usize {
        let last_line = if snippet.after.is_some() { snippet.line + 1 } else { snippet.line };
        last_line.to_string().len()
    }

    // Returns chopped string where first and third part are supposed
    // to be left as is but the second one is supposed to be highlighted
    fn get_highlighted_part(&self, code: &str) -> Option<[String;3]> {
        let (_row, col, len) = self.get_row_col_len()?;
        let begin = col.saturating_sub(1);
        let end = begin + len;
        let mut results: [String; 3] = Default::default();
        for (index, letter) in code.chars().enumerate() {
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

    // Return the line that the message points to with the highlighted part.
    // Also returns how many highlighted characters overflow to the next line.
    fn get_current_row(&self, code: &str, line: &str) -> Option<(String, usize)> {
        let (_row, col, len) = self.get_row_col_len()?;
        let slices = self.get_highlighted_part(code)?;
        let formatted = format!("{}{}{}", slices[0], slices[1].color(self.kind_to_color()), slices[2]);
        let end = col.checked_add(len).unwrap_or(len);
        let mut overflow = 0;
        // If we are at the end of the line and there is still some highlighted part left
        if end.saturating_sub(1) > code.chars().count() {
            // We subtract here 2 because 1 is the offset of col (starts at 1)
            // and other 1 is the new line character that we do not display
            overflow = end.saturating_sub(2).saturating_sub(code.chars().count());
        }
        Some((format!("{line}| {formatted}"), overflow))
    }

    // Return the surrounding line with the overflowing highlight (if there is any)
    fn get_context_row(&self, code: &str, line: &str, overflow: usize) -> String {
        let formatted = if overflow > 0 {
            let err = code.chars().take(overflow).collect::<String>();
            let rest = code.chars().skip(overflow).collect::<String>();
            format!("{}{rest}", err.color(self.kind_to_color()))
        } else {
            code.to_string()
        };
        format!("{line}| {formatted}").dimmed().to_string()
    }

    /// Render snippet of the code if the message is contextual to it
    pub fn snippet(self, snippet: Option<&Snippet>) -> Self {
        if let Some(snippet) = snippet {
            self.render_snippet(snippet);
        }
        self
    }

    fn render_snippet(&self, snippet: &Snippet) -> Option<()> {
        let pad = Self::get_pad_size(snippet);
        let line_number = |line: usize| line.to_string().pad_to_width(pad);
        let (current, overflow) = self.get_current_row(&snippet.current, &line_number(snippet.line))?;
        eprintln!();
        // Show additional code above the snippet
        if let Some(before) = &snippet.before {
            eprintln!("{}", self.get_context_row(before, &line_number(snippet.line - 1), 0));
        }
        // Show the current line of code
        eprintln!("{current}");
        // Show additional code below the snippet
        if let Some(after) = &snippet.after {
            eprintln!("{}", self.get_context_row(after, &line_number(snippet.line + 1), overflow));
        }
        Some(())
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use std::time::Duration;
    use std::thread::sleep;

    use crate::prelude::{DefaultMetadata, Message, Metadata, MessageType, PositionInfo, Token};
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
            PositionInfo::at_pos(Some("/path/to/bar"), (3, 4), 10, 10),
            PositionInfo::at_pos(Some("/path/to/foo"), (2, 9), 24, 24),
        ];
        super::Logger::new(MessageType::Error, &trace)
            .header(MessageType::Error)
            .line(Some(format!("Cannot call function \"foobar\" on a number")))
            .path()
            .snippet(Message::get_snippet(&code, &trace[0]).as_ref());
    }

    #[test]
    fn test_end_of_line_displayer() {
        let code = vec![
            "hello"
        ].join("\n");
        // Uncomment to see the error message
        sleep(Duration::from_secs(1));
        let trace = [
            PositionInfo::at_pos(Some("/path/to/foo"), (1, 6), 5, 1)
        ];
        super::Logger::new(MessageType::Error, &trace)
            .header(MessageType::Error)
            .line(Some(format!("Cannot call function \"foobar\" on a number")))
            .path()
            .snippet(Message::get_snippet(&code, &trace[0]).as_ref());
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
        let mut meta = DefaultMetadata::new(vec![], code.clone());
        let trace = [
            PositionInfo::from_between_tokens(&mut meta, Some(begin), Some(end))
        ];
        super::Logger::new(MessageType::Error, &trace)
            .header(MessageType::Error)
            .line(Some(format!("Cannot call function \"foobar\" on a number")))
            .path()
            .snippet(Message::get_snippet(&code, &trace[0]).as_ref());
    }
}
