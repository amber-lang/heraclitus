use std::fmt::{Formatter, Display, Result};

pub struct Token <'a> {
    pub word: String,
    pub path: &'a String,
    pub row: usize,
    pub col: usize
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let word = match self.word.as_str() {
            "\n" => String::from("<new_line>"),
            "\t" => String::from("<tab>"),
            " " => String::from("<space>"),
            sym @ (
                "[" | "]" |
                "<" | ">" |
                "{" | "}" |
                "(" | ")" |
                ":" | ";"
            ) => format!("<symbol: {} >", sym),
            _ => self.word.clone()
        };
        write!(formatter, "Tok[{} {}:{}]", word, self.row, self.col)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display_token() {
        let path = "/path/to/my/file".to_string();
        let mut token = super::Token {
            word: String::from("keyword"),
            path: &path,
            row: 1,
            col: 2
        };
        assert_eq!(format!("{}", token), String::from("Tok[keyword 1:2]"));

        token.word = String::from("[");
        assert_eq!(format!("{}", token), String::from("Tok[<symbol: [ > 1:2]"));
    }
}