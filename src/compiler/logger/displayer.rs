#![allow(dead_code)]
use colored::Colorize;
use pad::PadStr;
use crate::compiler::logger::LogType;
pub struct Displayer {
    color: (u8, u8, u8),
    row: usize,
    col: usize,
    len: usize
}

impl Displayer {
    /// Create a new Displayer instance
    pub fn new(color: (u8, u8, u8), row: usize, col: usize, len: usize) -> Self {
        Displayer {
            color,
            row,
            col,
            len
        }
    }

    /// Render header of your information
    pub fn header(self, kind: LogType) -> Self {
        let (r, g, b) = self.color;
        let name = match kind {
            LogType::Error => format!(" ERROR "),
            LogType::Warning => format!(" WARN "),
            LogType::Info => format!(" INFO ")
        };
        let formatted = name
            .truecolor(0, 0, 0)
            .bold()
            .on_truecolor(r, g, b);
        println!("{formatted}");
        self
    }

    /// Render text with supplied coloring
    pub fn text(self, text: Option<String>) -> Self {
        let (r, g, b) = self.color;
        if let Some(text) = text {
            println!("{}", text.truecolor(r, g, b));
        }
        self
    }

    /// Render padded text with supplied coloring
    pub fn padded_text(self, text: Option<String>) -> Self {
        let (r, g, b) = self.color;
        if let Some(text) = text {
            println!("\n{}", text.truecolor(r, g, b));
        }
        self
    }

    /// Render location details with supplied coloring
    pub fn path(self, path: Option<String>) -> Self {
        let path = path.unwrap_or(format!("[unknown]"));
        let (r, g, b) = self.color;
        let formatted = format!("at {}:{}:{}", path, self.row, self.col)
            .truecolor(r, g, b)
            .dimmed();
        println!("{formatted}");
        self
    }

    // Get max padding size (for line numbering)
    fn get_max_pad_size(&self, length: usize) -> usize {
        if self.row < length - 1 {
            format!("{}", self.row + 1).len()
        }
        else {
            format!("{}", self.row).len()
        }
    }

    // Returns chopped string where fisrt and third part are supposed
    // to be left as is but the second one is supposed to be highlighted
    fn get_highlighted_part(&self, line: &String) -> [String;3] {
        let begin = self.col - 1;
        let end = begin + self.len;
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
        results
    }

    // Return requested row with appropriate coloring
    fn get_snippet_row(&self, code: &Vec<String>, index: usize, offset: i8, overflow: &mut usize) -> String {
        let max_pad = self.get_max_pad_size(code.len());
        let index = index as i32 + offset as i32;
        let row = self.row as i32 + offset as i32;
        let code = code[index as usize].clone();
        let line = format!("{row}").pad_to_width(max_pad);
        let (r, g, b) = self.color;
        // Case if we are in the same line as the error (or message)
        if offset == 0 {
            let slices = self.get_highlighted_part(&code);
            let formatted = format!("{}{}{}", slices[0], slices[1].truecolor(r, g, b), slices[2]);
            // If we are at the end of the code snippet and there is still some
            if self.col - 1 + self.len > code.chars().count() {
                // We substract here 2 because 1 is the offset of col (starts at 1)
                // and other 1 is the new line character that we do not display
                *overflow = self.col - 2 + self.len - code.chars().count();
            }
            format!("{}", format!("{line}| {formatted}"))
        }
        // Case if we are in a different line than the error (or message)
        else {
            // If there is some overflow value - display it as well
            if *overflow > 0 {
                // Case if all line is highlighted
                if *overflow > code.chars().count() {
                    format!("{}", format!("{line}| {}", code.truecolor(r, g, b)).dimmed())
                }
                // Case if some line is highlighted
                else {
                    let err = code.get(0..*overflow).unwrap().to_string().truecolor(r, g, b);
                    let rest = code.get(*overflow..).unwrap().to_string();
                    format!("{}", format!("{line}| {err}{rest}").dimmed())
                }
            }
            // Case if no overflow
            else {
                format!("{}", format!("{line}| {code}").dimmed())
            }
        }
    }

    /// Render snippet of the code if the message is contextual to it
    pub fn snippet<T: AsRef<str>>(self, code: Option<T>) {
        if let Some(code) = code {
            let mut overflow = 0;
            let index = self.row - 1;
            let code: String = String::from(code.as_ref());
            let code = code.split("\n")
                .map(|item| item.to_string())
                .collect::<Vec<String>>();
            println!("");
            // Show additional code above the snippet
            if index > 0 {
                println!("{}", self.get_snippet_row(&code, index, -1, &mut overflow));
            }
            println!("{}", self.get_snippet_row(&code, index, 0, &mut overflow));
            // Show additional code below the snippet
            if index < code.len() - 1 {
                println!("{}", self.get_snippet_row(&code, index, 1, &mut overflow));
            }
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use std::time::Duration;
    use std::thread::sleep;
    #[allow(unused_variables)]
    
    #[test]
    fn test_displayer() {
        let code = vec![
            "let a = 12",
            "value = 'this",
            "is mutltiline",
            "code"
        ].join("\n");
        // Uncomment to see the error message
        // sleep(Duration::from_secs(1));
        // super::Displayer::new((255, 80, 80), 2, 9, 24)
        //     .header(super::LogType::Error)
        //     .text(Some(format!("Cannot call function \"foobar\" on a number")))
        //     .path(Some(format!("/path/to/file")))
        //     .snippet(Some(code));
    }
}