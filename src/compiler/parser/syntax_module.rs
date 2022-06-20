use crate::compiler::Token;

pub enum PresetKind {
    Variable,
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
                SyntaxSymbol::Token(text) => {
                    match expr.get(index) {
                        Some(token) => {
                            if token.word != String::from(*text) {
                                return false;
                            }
                            index += 1;
                        }
                        None => return false
                    }
                },
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

}
