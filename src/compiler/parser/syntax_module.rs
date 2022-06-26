use crate::compiler::Token;
use super::pattern::*;
use super::preset;

macro_rules! pat {
    ($($exp:expr),*) => {
        SyntaxSymbol::Pattern(vec![$($exp),*])
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
        SyntaxSymbol::Repeat(Box::new($pat), Box::new(SyntaxSymbol::Pattern(vec![])))
    };
}

macro_rules! tok {
    ($exp:expr) => {
        SyntaxSymbol::Token(format!("{}", $exp))
    };
}

macro_rules! pre {
    ($name:ident) => {
        SyntaxSymbol::Preset(PresetKind::$name(vec![]))
    };
    ($name:ident, [$($params:expr),*]) => {
        SyntaxSymbol::Preset(PresetKind::$name(vec![$($params),*]))
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

#[derive(Clone)]
pub enum PresetKind {
    Variable(Vec<char>),
    Alphabetic(Vec<char>),
    Alphanumeric(Vec<char>),
    Numeric(Vec<char>),
    Number(Vec<char>),
    Integer(Vec<char>),
    Float(Vec<char>)
}

#[derive(Clone)]
pub enum SyntaxSymbol<'a> {
    Token(String),
    Preset(PresetKind),
    Pattern(Vec<SyntaxSymbol<'a>>),
    Any(Vec<SyntaxSymbol<'a>>),
    Optional(Box<SyntaxSymbol<'a>>),
    Repeat(Box<SyntaxSymbol<'a>>, Box<SyntaxSymbol<'a>>),
    Syntax(Box<&'a dyn SyntaxModule>),
    IndentBlock(Box<SyntaxSymbol<'a>>),
    Custom(fn(&[Token]) -> Option<usize>)
}

pub struct SyntaxMetadata {
    pub index: usize,
    pub is_indent: bool,
    pub indent_level: usize,
    pub indent_stack: Vec<usize>
}

impl SyntaxMetadata {
    fn new() -> Self {
        SyntaxMetadata {
            index: 0,
            is_indent: false,
            indent_level: 0,
            indent_stack: vec![]
        }
    }
}

pub enum SyntaxResult {
    Word(String),
    Module(),
    Pattern(Vec<SyntaxResult>),
    Custom(Vec<String>),
    Optional(Option<Box<SyntaxResult>>)
}

pub trait SyntaxModule {
    // Recursively match syntax symbol
    fn match_pattern_recursive(&self, expr: &[Token], meta: &mut SyntaxMetadata, symbol: &SyntaxSymbol) -> Option<SyntaxResult> {
        match symbol {
            // Match token - check if next token matches the string
            SyntaxSymbol::Token(text) => match_token(text, expr, meta),
            // Match preset - check if the token matches one of presets
            SyntaxSymbol::Preset(preset) => {
                match preset {
                    PresetKind::Variable(extend) => preset::match_variable(expr, meta, extend),
                    PresetKind::Alphabetic(extend) => preset::match_alphabetic(expr, meta, extend),
                    PresetKind::Alphanumeric(extend) => preset::match_alphanumeric(expr, meta, extend),
                    PresetKind::Numeric(extend) => preset::match_numeric(expr, meta, extend),
                    PresetKind::Number(extend) => preset::match_number(expr, meta, extend),
                    PresetKind::Integer(extend) => preset::match_integer(expr, meta, extend),
                    PresetKind::Float(extend) => preset::match_float(expr, meta, extend)
                }
            },
            // Match one of the options
            SyntaxSymbol::Any(options) => {
                for option in options.iter() {
                    if let Some(matched) = self.match_pattern_recursive(expr, meta, option) {
                        return Some(matched)
                    }
                }
                None
            },
            // Match all elements in the pattern
            SyntaxSymbol::Pattern(pattern) => {
                let mut result = vec![];
                let old_index = meta.index.clone();
                for pattern in pattern.iter() {
                    if let Some(matched) = self.match_pattern_recursive(expr, meta, pattern) {
                        result.push(matched);
                        continue
                    }
                    else {
                        meta.index = old_index;
                        return None
                    }
                }
                Some(SyntaxResult::Pattern(result))
            },
            // Symbol that can happen but doesn't have to
            SyntaxSymbol::Optional(symbol) => {
                if let Some(result) = self.match_pattern_recursive(expr, meta, symbol) {
                    Some(SyntaxResult::Optional(Some(Box::new(result))))
                } else { Some(SyntaxResult::Optional(None)) }
            },
            // Match repeating pattern
            SyntaxSymbol::Repeat(pattern, separator) => {
                let mut result = vec![];
                // Merge separator and pattern
                let both = SyntaxSymbol::Pattern(vec![*separator.clone(), *pattern.clone()]);
                // Match first element
                if let Some (matched) = self.match_pattern_recursive(expr, meta, pattern) {
                    result.push(matched);
                } else { return Some(SyntaxResult::Pattern(result))}
                // Match more elements
                loop {
                    if let Some(matched) = self.match_pattern_recursive(expr, meta, &both) {
                        if let SyntaxResult::Pattern(matched_pattern) = matched {
                            for pattern in matched_pattern {
                                result.push(pattern);
                            }
                        }
                    } else { return Some(SyntaxResult::Pattern(result)) }
                }
            },
            // Match other syntax module
            SyntaxSymbol::Syntax(module) => {
                if let Some(_result) = module.match_pattern(expr, meta) {
                    return Some(SyntaxResult::Module())
                } else { None }
            },
            // Match custom expression
            SyntaxSymbol::Custom(function) => {
                if let Some(new_index) = function(&expr[meta.index..]) {
                    let old_index = meta.index;
                    meta.index += new_index;
                    Some(SyntaxResult::Custom(expr[old_index..meta.index].iter().map(|item| item.word.clone()).collect()))
                } else { None }
            }
            _ => None
        }
    }

    fn match_pattern(&self, expr: &[Token], meta: &mut SyntaxMetadata) -> Option<SyntaxResult> {
        let symbol = self.pattern();
        self.match_pattern_recursive(expr, meta, &symbol)
    }

    fn parse_pattern(&self) {
        unimplemented!()
    }

    fn pattern<'a>(&self) -> SyntaxSymbol<'a>;
}

#[cfg(test)]
mod test {
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
        let result1 = exp.match_pattern(&dataset1[..], &mut SyntaxMetadata::new());
        let result2 = exp.match_pattern(&dataset2[..], &mut SyntaxMetadata::new());
        assert!(result1.is_some());
        assert!(result2.is_none());
    }

    struct Preset {}
    impl SyntaxModule for Preset {
        fn pattern<'a>(&self) -> SyntaxSymbol<'a> {
            pat![
                pre!(Variable, ['_']),
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
            Token { word: format!("-123.12"), path: path, pos: (0, 0) },
            // Integer
            Token { word: format!("-12"), path: path, pos: (0, 0) },
            // Float
            Token { word: format!("-.681"), path: path, pos: (0, 0)}
        ];
        let result = exp.match_pattern(&dataset[..], &mut SyntaxMetadata::new());
        assert!(result.is_some());
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
    struct PatternModule {}
    impl SyntaxModule for PatternModule {
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
        let exp = PatternModule {};
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
        let result1 = exp.match_pattern(&dataset1[..], &mut SyntaxMetadata::new());
        let result2 = exp.match_pattern(&dataset2[..], &mut SyntaxMetadata::new());
        let result3 = exp.match_pattern(&dataset3[..], &mut SyntaxMetadata::new());
        let result4 = exp.match_pattern(&dataset4[..], &mut SyntaxMetadata::new());
        let result5 = exp.match_pattern(&dataset5[..], &mut SyntaxMetadata::new());
        assert!(result1.is_some());
        assert!(result2.is_none());
        assert!(result3.is_none());
        assert!(result4.is_none());
        assert!(result5.is_none());
    }

}
