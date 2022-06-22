use crate::compiler::Token;
use super::pattern::*;
use super::preset;

pub enum PresetKind {
    Variable,
    Numeric,
    Number,
    Integer,
    Float
}

pub enum SyntaxSymbol<'a> {
    And(Vec<SyntaxSymbol<'a>>),
    Or(Vec<SyntaxSymbol<'a>>),
    Optional(Box<SyntaxSymbol<'a>>),
    Token(&'a str),
    Preset(PresetKind),
    SyntaxModule(&'a dyn SyntaxModule),
    Block(&'a dyn SyntaxModule),
    Custom(fn(&[Token]) -> (bool, usize))
}

pub trait SyntaxModule {

    // Recursively match syntax symbol
    fn match_pattern_recursive(&self, expr: &[Token], index: &mut usize, symbol: &SyntaxSymbol) -> bool {
        match symbol {
            // Match token - check if next token matches the string
            SyntaxSymbol::Token(text) => match_token(text, expr, index),
            // Match preset - check if the token matches one of presets
            SyntaxSymbol::Preset(preset) => {
                match preset {
                    PresetKind::Variable => preset::match_variable(expr, index),
                    PresetKind::Numeric => preset::match_numeric(expr, index),
                    PresetKind::Number => preset::match_number(expr, index),
                    PresetKind::Integer => preset::match_integer(expr, index),
                    PresetKind::Float => preset::match_float(expr, index)
                }
            },
            // Match one of the options
            SyntaxSymbol::Or(options) => {
                for option in options.iter() {
                    if self.match_pattern_recursive(expr, index, option) {
                        return true;
                    }
                }
                false
            },
            // Match all of the options
            SyntaxSymbol::And(options) => {
                for option in options.iter() {
                    if !self.match_pattern_recursive(expr, index, option) {
                        return false;
                    }
                }
                true
            },
            SyntaxSymbol::Optional(symbol) => {
                self.match_pattern_recursive(expr, index, symbol);
                true
            }
            _ => true
        }
    }

    // Match pattern
    fn match_pattern(&self, expr: &[Token]) -> bool {
        let mut index: usize = 0;
        let symbol = self.pattern();
        self.match_pattern_recursive(expr, &mut index, &symbol)
    }

    fn pattern<'a>(&self) -> SyntaxSymbol<'a>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct Expression {}
    impl SyntaxModule for Expression {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            use SyntaxSymbol::*;
            And(vec![
                Token("let")
            ])
        }
    }

    #[test]
    fn test_token_match() {
        let exp = Expression {};
        let path = &format!("/path/to/file");
        let dataset1 = vec![
            Token {
                word: format!("let"),
                path: path,
                pos: (0, 0)
            }
        ];
        let dataset2 = vec![
            Token {
                word: format!("tell"),
                path: path,
                pos: (0, 0)
            }
        ];
        let result1 = exp.match_pattern(&dataset1[..]);
        let result2 = exp.match_pattern(&dataset2[..]);
        assert!(result1);
        assert!(!result2);
    }

    struct Preset {}
    impl SyntaxModule for Preset {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            use SyntaxSymbol::*;
            use PresetKind::*;
            And(vec![
                Preset(Variable), Preset(Numeric), Preset(Number), Preset(Integer), Preset(Float)
            ])
        }
    }

    #[test]
    fn test_preset_match() {
        let exp = Preset {};
        let path = &format!("/path/to/file");
        let dataset = vec![
            // Variable
            Token { word: format!("text"), path: path, pos: (0, 0) },
            // Numeric
            Token { word: format!("12321"), path: path, pos: (0, 0) },
            // Number
            Token { word: format!("-"), path: path, pos: (0, 0) },
            Token { word: format!("123"), path: path, pos: (0, 0) },
            Token { word: format!("."), path: path, pos: (0, 0) },
            Token { word: format!("12"), path: path, pos: (0, 0) },
            // Integer
            Token { word: format!("-"), path: path, pos: (0, 0) },
            Token { word: format!("12"), path: path, pos: (0, 0) },
            // Float
            Token { word: format!("-"), path: path, pos: (0, 0)},
            Token { word: format!("."), path: path, pos: (0, 0) },
            Token { word: format!("681"), path: path, pos: (0, 0) }
        ];
        let result = exp.match_pattern(&dataset[..]);
        assert!(result);
    }

    struct OrAndOptional {}
    impl SyntaxModule for OrAndOptional {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            use SyntaxSymbol::*;
            And(vec![
                Or(vec![
                    Token("apple"),
                    Token("orange"),
                    Token("banana")
                ]),
                Optional(Box::new(Token("optional"))),
                Token("end")
            ])
        }
    }

    #[test]
    fn test_or_and_optional_match() {
        let exp = OrAndOptional {};
        let path = &format!("/path/to/file");
        let dataset = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("optional"), path: path, pos: (0, 0) },
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        let result = exp.match_pattern(&dataset[..]);
        assert!(result);
    }

}
