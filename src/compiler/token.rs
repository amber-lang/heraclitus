use std::fmt::{Formatter, Display, Result, Debug};

#[derive(Clone)]
pub struct Token {
    pub word: String,
    pub pos: (usize, usize),
}

impl Token {
    fn format(&self, formatter: &mut Formatter) -> Result {
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
        write!(formatter, "Tok[{} {}:{}]", word, self.pos.0, self.pos.1)
    }
}

impl Display for Token {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        self.format(formatter)
    }
}

impl Debug for Token {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        self.format(formatter)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display_token() {
        let path = "/path/to/my/file".to_string();
        let mut token = super::Token {
            word: String::from("keyword"),
            pos: (1, 2)
        };
        assert_eq!(format!("{}", token), String::from("Tok[keyword 1:2]"));
        token.word = String::from("[");
        assert_eq!(format!("{}", token), String::from("Tok[<symbol: [ > 1:2]"));
    }
}