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
    Token(&'a str),
    Preset(PresetKind),
    SyntaxModule(&'a dyn SyntaxModule),
    Or(Vec<SyntaxSymbol<'a>>),
    Custom(fn(&[Token]) -> (bool, usize)),
    Block(&'a dyn SyntaxModule)
}

pub type SyntaxPattern<'a> = Vec<SyntaxSymbol<'a>>;

pub trait SyntaxModule {
    fn match_pattern(&self, expr: &[Token]) -> bool {
        let mut index: usize = 0;
        let symbols = self.pattern();
        for symbol in symbols.iter() {
            match symbol {
                // Match token - check if next token matches the string
                SyntaxSymbol::Token(text) => {
                    if !match_token(text, expr, &mut index) {
                        return false;
                    }
                },
                // Match preset - check if the token matches one of presets
                SyntaxSymbol::Preset(preset) => {
                    match preset {
                        PresetKind::Variable => {
                            if !preset::match_variable(expr, &mut index) {
                                return false;
                            }
                        },
                        PresetKind::Numeric => {
                            if !preset::match_numeric(expr, &mut index) {
                                return false;
                            }
                        },
                        PresetKind::Number => {
                            if !preset::match_number(expr, &mut index) {
                                return false;
                            }
                        },
                        PresetKind::Integer => {
                            if !preset::match_integer(expr, &mut index) {
                                return false;
                            }
                        },
                        PresetKind::Float => {
                            if !preset::match_float(expr, &mut index) {
                                return false;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn pattern<'a>(&self) -> SyntaxPattern<'a>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct Expression {}
    impl SyntaxModule for Expression {
        fn pattern<'a>(&self) -> SyntaxPattern<'a> {
            use SyntaxSymbol::*;
            vec![
                Token("let")
            ]
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
        fn pattern<'a>(&self) -> SyntaxPattern<'a> {
            use SyntaxSymbol::*;
            use PresetKind::*;
            vec![
                Preset(Variable), Preset(Numeric), Preset(Number), Preset(Integer), Preset(Float)
            ]
        }
    }

    #[test]
    fn test_preset_match() {
        let exp = Preset {};
        let path = &format!("/path/to/file");
        let dataset1 = vec![
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
        let result = exp.match_pattern(&dataset1[..]);
        assert!(result);
    }

}
