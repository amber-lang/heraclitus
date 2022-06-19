use crate::compiler::Token;

macro_rules! pattern {
    [$($items:expr),*] => [
        vec![$($items),*]
    ]
}

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
    fn match_pattern(&self, expression: &[Token]) {
        let mut symbols = self.pattern();
        for symbol in symbols.iter() {
            match symbol {
                SyntaxSymbol::Token(text) => {
                    // TODO: Finish
                },
                _ => {}
            }
        }
    }

    fn pattern<'a>(&self) -> SyntaxPattern<'a>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct Expression {}
    impl SyntaxModule for Expression {
        fn pattern<'a>(&self) -> SyntaxPattern<'a> {
            use PresetKind::*;
            use SyntaxSymbol::*;
            pattern![
                Token("let"), Preset(Variable), Token("=")
            ]
        }
    }

}
