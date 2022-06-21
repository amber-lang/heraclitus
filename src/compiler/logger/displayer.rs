use colored::Colorize;
use pad::PadStr;
use crate::compiler::logger::LogType;

pub struct Displayer {
    color: (u8, u8, u8),
    row: usize,
    col: usize
}

impl Displayer {
    pub fn new(color: (u8, u8, u8), row: usize, col: usize) -> Self {
        Displayer {
            color,
            row,
            col
        }
    }

    // Render header of your information
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

    // Render text with supplied coloring
    pub fn text(self, text: Option<String>) -> Self {
        let (r, g, b) = self.color;
        if let Some(text) = text {
            println!("{}", text.truecolor(r, g, b));
        }
        self
    }

    // Render padded text with supplied coloring
    pub fn padded_text(self, text: Option<String>) -> Self {
        let (r, g, b) = self.color;
        if let Some(text) = text {
            println!("\n{}", text.truecolor(r, g, b));
        }
        self
    }

    // Render location details with supplied coloring
    pub fn path(self, path: &String) -> Self {
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
    fn get_highlighted_part(&self, line: &String, length: usize) -> [String;3] {
        let begin = self.col - 1;
        let end = begin + length;
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
    fn get_snippet_row(&self, code: &Vec<String>, index: usize, length: usize, offset: i32) -> String {
        let max_pad = self.get_max_pad_size(code.len());
        let index = index as i32 + offset;
        let row = self.row as i32 + offset;
        let code = code[index as usize].clone();
        let line = format!("{row}").pad_to_width(max_pad);
        if offset == 0 {
            let (r, g, b) = self.color;
            let slices = self.get_highlighted_part(&code, length);
            let formatted = format!("{}{}{}", slices[0], slices[1].truecolor(r, g, b), slices[2]);
            format!("{}", format!("{line}| {formatted}"))
        } else {
            format!("{}", format!("{line}| {code}").dimmed())
        }
    }

    // Render snippet of the code if the message is contextual to it
    pub fn snippet<T: AsRef<str>>(self, code: T, length: usize) {
        let index = self.row - 1;
        let code: String = String::from(code.as_ref());
        let code = code.split("\n")
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        println!("");
        // Show additional code above the snippet
        if index > 0 {
            println!("{}", self.get_snippet_row(&code, index, length, -1));
        }
        println!("{}", self.get_snippet_row(&code, index, length, 0));
        // Show additional code below the snippet
        if index < code.len() - 1 {
            println!("{}", self.get_snippet_row(&code, index, length, 1));
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use std::thread::sleep;
    #[allow(unused_variables)]
    
    #[test]
    fn test_displayer() {
        let code = vec![
            "let a = 12",
            "a.run()",
            "a += 24"
        ].join("\n");
        sleep(Duration::from_secs(1));
        /* Uncomment to see the error message
        super::Displayer::new((255, 80, 80), 2, 3)
            .header(super::LogType::Error)
            .text(Some(format!("Cannot call function \"run\" on a number")))
            .path(&format!("/path/to/file"))
            .snippet(code, 3);
        */
    }
}