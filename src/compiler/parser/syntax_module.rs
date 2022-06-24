use crate::compiler::Token;
use super::pattern::*;
use super::preset;

macro_rules! pat {
    ($($exp:expr),*) => {
        SyntaxSymbol::And(vec![$($exp),*])
    };
}

macro_rules! any {
    ($($exp:expr),*) => {
        SyntaxSymbol::Any(vec![$($exp),*])
    };
}

macro_rules! rep {
    ($pat:expr, $sep:expr) => {
        SyntaxSymbol::Repeat(Box::new($pat), Box::new($sep))
    };
    ($pat:expr) => {
        SyntaxSymbol::Repeat(Box::new($pat), Box::new(SyntaxSymbol::And(vec![])))
    };
}

macro_rules! tok {
    ($exp:expr) => {
        SyntaxSymbol::Token(format!("{}", $exp))
    };
}

macro_rules! pre {
    ($name:ident) => {
        SyntaxSymbol::Preset(PresetKind::$name)
    };
}

macro_rules! opt {
    ($exp:expr) => {
        SyntaxSymbol::Optional(Box::new($exp))
    };
}

macro_rules! cus {
    ($exp:expr) => {
        SyntaxSymbol::Custom($exp)
    };
}

macro_rules! syn {
    ($exp:expr) => {
        SyntaxSymbol::Syntax(Box::new($exp))
    };
}

// TODO: Create block pattern
macro_rules! blc {
    ($exp:expr) => {
        SyntaxSymbol::Custom($exp)
    };
}

// TODO: Remove this temporary function
fn test() {
    pat![
        any![
            tok!("static"),
            tok!("async")
        ],
        tok!("fun"),
        pre!(Variable),
        opt!(tok!("macro")),
        tok!("("),
        rep!(pre!(Variable), tok!(",")),
        tok!(")")
    ];
}

#[derive(Clone)]
pub enum PresetKind {
    Variable,
    Numeric,
    Number,
    Integer,
    Float
}

#[derive(Clone)]
pub enum SyntaxSymbol<'a> {
    Token(String),
    Preset(PresetKind),
    And(Vec<SyntaxSymbol<'a>>),
    Any(Vec<SyntaxSymbol<'a>>),
    Optional(Box<SyntaxSymbol<'a>>),
    Repeat(Box<SyntaxSymbol<'a>>, Box<SyntaxSymbol<'a>>),
    Syntax(Box<&'a dyn SyntaxModule>),
    Block(Box<SyntaxSymbol<'a>>),
    Custom(fn(&[Token]) -> Option<usize>)
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
            SyntaxSymbol::Any(options) => {
                for option in options.iter() {
                    if self.match_pattern_recursive(expr, index, option) {
                        return true;
                    }
                }
                false
            },
            // Match all elements in the pattern
            SyntaxSymbol::And(pattern) => {
                let mut new_index = index.clone();
                for pattern in pattern.iter() {
                    if !self.match_pattern_recursive(expr, &mut new_index, pattern) {
                        return false;
                    }
                }
                *index = new_index;
                true
            },
            // Symbol that can happen but doesn't have to
            SyntaxSymbol::Optional(symbol) => {
                self.match_pattern_recursive(expr, index, symbol);
                true
            },
            // Match repeating pattern
            SyntaxSymbol::Repeat(pattern, separator) => {
                let both = SyntaxSymbol::And(vec![*separator.clone(), *pattern.clone()]);
                self.match_pattern_recursive(expr, index, pattern);
                loop {
                    if !self.match_pattern_recursive(expr, index, &both) {
                        return true;
                    }
                }
            },
            // Match other syntax module
            SyntaxSymbol::Syntax(module) => {
                module.match_pattern(expr, index)
            },
            // Match custom expression
            SyntaxSymbol::Custom(function) => {
                if let Some(new_index) = function(&expr[*index..]) {
                    *index += new_index;
                    true
                } else { false }
            },
            _ => true
        }
    }

    fn match_pattern(&self, expr: &[Token], index: &mut usize) -> bool {
        let symbol = self.pattern();
        self.match_pattern_recursive(expr, index, &symbol)
    }

    fn pattern<'a>(&self) -> SyntaxSymbol<'a>;
}

#[cfg(test)]
mod test {
    use std::char::ToLowercase;

    use super::*;

    struct Expression {}
    impl SyntaxModule for Expression {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            tok!["let"]
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
        let result1 = exp.match_pattern(&dataset1[..], &mut 0);
        let result2 = exp.match_pattern(&dataset2[..], &mut 0);
        assert!(result1);
        assert!(!result2);
    }

    struct Preset {}
    impl SyntaxModule for Preset {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            pat![
                pre!(Variable),
                pre!(Numeric),
                pre!(Number),
                pre!(Integer),
                pre!(Float)
            ]
        }
    }

    #[test]
    fn test_preset_match() {
        let exp = Preset {};
        let path = &format!("/path/to/file");
        let dataset = vec![
            // Variable
            Token { word: format!("_text"), path: path, pos: (0, 0) },
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
        let result = exp.match_pattern(&dataset[..], &mut 0);
        assert!(result);
    }

    // Function that can be used to express custom pattern
    fn my_custom_pattern(expr: &[Token]) -> Option<usize> {
        match expr.get(0) {
            Some(token) => {
                for letter in token.word.chars() {
                    if !letter.is_alphabetic() {
                        return None
                    }
                }
                Some(1)
            }
            None => None
        }
    }
    struct AnyAndOptional {}
    impl SyntaxModule for AnyAndOptional {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            pat![
                any![
                    tok!("apple"),
                    tok!("orange"),
                    tok!("banana")
                ],
                opt!(tok!("optional")),
                syn!(&Expression {}),
                rep!(tok!("this"), tok!(",")),
                cus!(my_custom_pattern),
                tok!("end")
                
            ]
        }
    }

    #[test]
    fn rest_match() {
        let exp = AnyAndOptional {};
        let path = &format!("/path/to/file");
        // Everything should pass
        let dataset1 = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("optional"), path: path, pos: (0, 0) },
            Token { word: format!("let"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!("abc"), path: path, pos: (0, 0) },
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        // Token should fail
        let dataset2 = vec![
            Token { word: format!("kiwi"), path: path, pos: (0, 0) },
            Token { word: format!("optional"), path: path, pos: (0, 0) },
            Token { word: format!("let"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!("abc"), path: path, pos: (0, 0) },
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        // Syntax should fail
        let dataset3 = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("tell"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!("abc"), path: path, pos: (0, 0) },
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        // Token should fail because of repeat matching (this , this) ,
        let dataset4 = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("tell"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("abc"), path: path, pos: (0, 0) },
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        // Custom pattern should fail because passed word is not an alphabetical word
        let dataset5 = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("tell"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!("_abc"), path: path, pos: (0, 0) },
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        let result1 = exp.match_pattern(&dataset1[..], &mut 0);
        let result2 = exp.match_pattern(&dataset2[..], &mut 0);
        let result3 = exp.match_pattern(&dataset3[..], &mut 0);
        let result4 = exp.match_pattern(&dataset4[..], &mut 0);
        let result5 = exp.match_pattern(&dataset5[..], &mut 0);
        assert!(result1);
        assert!(!result2);
        assert!(!result3);
        assert!(!result4);
        assert!(!result5);
    }

}
