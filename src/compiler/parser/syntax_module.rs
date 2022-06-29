use crate::compiler::Token;

// #[derive(Clone)]
// pub enum PresetKind {
//     Variable(Vec<char>),
//     Alphabetic(Vec<char>),
//     Alphanumeric(Vec<char>),
//     Numeric(Vec<char>),
//     Number(Vec<char>),
//     Integer(Vec<char>),
//     Float(Vec<char>)
// }

// #[derive(Clone)]
// pub enum SyntaxSymbol {
//     Token(String),
//     Preset(PresetKind),
//     Pattern(Vec<SyntaxSymbol>),
//     Any(Vec<SyntaxSymbol>),
//     Optional(Box<SyntaxSymbol>),
//     Repeat(Box<SyntaxSymbol>, Box<SyntaxSymbol>),
//     Syntax(Box<dyn SyntaxModule>),
//     IndentBlock(Box<SyntaxSymbol>),
//     Custom(fn(&[Token]) -> Option<usize>)
// }

pub struct SyntaxMetadata<'a> {
    pub index: usize,
    pub is_indent: bool,
    pub indent_level: usize,
    pub indent_stack: Vec<usize>,
    pub expr: &'a [Token<'a>]
}

impl<'a> SyntaxMetadata<'a> {
    fn new(expression: &'a [Token]) -> Self {
        SyntaxMetadata {
            index: 0,
            is_indent: false,
            indent_level: 0,
            indent_stack: vec![],
            expr: expression
        }
    }
}

pub type SyntaxResult = Result<(),()>;

pub trait SyntaxModule {
    // Recursively match syntax symbol
    // fn match_pattern_recursive(&self, expr: &[Token], meta: &mut SyntaxMetadata, symbol: &SyntaxSymbol) -> Option<SyntaxResult> {
    //     match symbol {
    //         // Match token - check if next token matches the string
    //         SyntaxSymbol::Token(text) => match_token(text, meta),
    //         // Match preset - check if the token matches one of presets
    //         SyntaxSymbol::Preset(preset) => {
    //             match preset {
    //                 PresetKind::Variable(extend) => preset::match_variable(expr, meta, extend),
    //                 PresetKind::Alphabetic(extend) => preset::match_alphabetic(expr, meta, extend),
    //                 PresetKind::Alphanumeric(extend) => preset::match_alphanumeric(expr, meta, extend),
    //                 PresetKind::Numeric(extend) => preset::match_numeric(expr, meta, extend),
    //                 PresetKind::Number(extend) => preset::match_number(expr, meta, extend),
    //                 PresetKind::Integer(extend) => preset::match_integer(expr, meta, extend),
    //                 PresetKind::Float(extend) => preset::match_float(expr, meta, extend)
    //             }
    //         },
    //         // Match one of the options
    //         SyntaxSymbol::Any(options) => {
    //             for option in options.iter() {
    //                 if let Some(matched) = self.match_pattern_recursive(expr, meta, option) {
    //                     return Some(matched)
    //                 }
    //             }
    //             None
    //         },
    //         // Match all elements in the pattern
    //         SyntaxSymbol::Pattern(pattern) => {
    //             let mut result = vec![];
    //             let old_index = meta.index.clone();
    //             for pattern in pattern.iter() {
    //                 if let Some(matched) = self.match_pattern_recursive(expr, meta, pattern) {
    //                     result.push(matched);
    //                     continue
    //                 }
    //                 else {
    //                     meta.index = old_index;
    //                     return None
    //                 }
    //             }
    //             Some(SyntaxResult::Pattern(result))
    //         },
    //         // Symbol that can happen but doesn't have to
    //         SyntaxSymbol::Optional(symbol) => {
    //             if let Some(result) = self.match_pattern_recursive(expr, meta, symbol) {
    //                 Some(SyntaxResult::Optional(Some(Box::new(result))))
    //             } else { Some(SyntaxResult::Optional(None)) }
    //         },
    //         // Match repeating pattern
    //         SyntaxSymbol::Repeat(pattern, separator) => {
    //             let mut result = vec![];
    //             // Merge separator and pattern
    //             let both = SyntaxSymbol::Pattern(vec![*separator.clone(), *pattern.clone()]);
    //             // Match first element
    //             if let Some (matched) = self.match_pattern_recursive(expr, meta, pattern) {
    //                 result.push(matched);
    //             } else { return Some(SyntaxResult::Pattern(result))}
    //             // Match more elements
    //             loop {
    //                 if let Some(matched) = self.match_pattern_recursive(expr, meta, &both) {
    //                     if let SyntaxResult::Pattern(matched_pattern) = matched {
    //                         for pattern in matched_pattern {
    //                             result.push(pattern);
    //                         }
    //                     }
    //                 } else { return Some(SyntaxResult::Pattern(result)) }
    //             }
    //         },
    //         // Match other syntax module
    //         SyntaxSymbol::Syntax(module) => {
    //             if let Some(result) = module.match_pattern(expr, meta) {
    //                 module.parse(result);
    //                 // Some(SyntaxResult::Module(Box::new(module.as_any())))
    //                 None
    //             } else { None }
    //         },
    //         // Match custom expression
    //         SyntaxSymbol::Custom(function) => {
    //             if let Some(new_index) = function(&expr[meta.index..]) {
    //                 let old_index = meta.index;
    //                 meta.index += new_index;
    //                 Some(SyntaxResult::Custom(expr[old_index..meta.index].iter().map(|item| item.word.clone()).collect()))
    //             } else { None }
    //         }
    //         _ => None
    //     }
    // }
    // 
    // fn match_pattern(&self, expr: &[Token], meta: &mut SyntaxMetadata) -> Option<SyntaxResult> {
    //     let symbol = self.pattern();
    //     self.match_pattern_recursive(expr, meta, &symbol)
    // }

    fn parse(&self, meta: &mut SyntaxMetadata) -> SyntaxResult;
}

#[cfg(test)]
mod test {
    use std::os::macos::fs::MetadataExt;

    use super::*;
    use crate::compiler::parser::pattern::*;
    use crate::compiler::parser::preset::*;

    struct Expression {}
    impl SyntaxModule for Expression {
        fn parse(&self, meta: &mut SyntaxMetadata) -> SyntaxResult {
            token(meta, "let")?;
            Ok(())
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
        let result1 = exp.parse(&mut SyntaxMetadata::new(&dataset1));
        let result2 = exp.parse(&mut SyntaxMetadata::new(&dataset2));
        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    struct Preset {}
    impl SyntaxModule for Preset {
        fn parse(&self, meta: &mut SyntaxMetadata) -> SyntaxResult {
            variable(meta, vec!['_'])?;
            numeric(meta, vec![])?;
            number(meta, vec![])?;
            integer(meta, vec![])?;
            float(meta, vec![])?;
            Ok(())
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
        let result = exp.parse(&mut SyntaxMetadata::new(&dataset));
        assert!(result.is_ok());
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
        fn pattern(&self) -> SyntaxSymbol {
            pat![
                any![
                    tok!("apple"),
                    tok!("orange"),
                    tok!("banana")
                ],
                opt!(tok!("optional")),
                syn!(&Expression {}),
                rep!(tok!("this"), tok!(",")),
                tok!("end")
            ]
        }

        fn parse(&self, meta: &mut SyntaxMetadata) -> SyntaxResult {
            // Any
            if let Ok(_) = token(meta, "apple") {}
            else if let Ok(_) = token(meta, "orange") {}
            else if let Ok(_) = token(meta, "banana") {}
            else { return Err(()) }
            // Optional
            token(meta, "optional");
            // Syntax
            syntax(meta, Expression {})?;
            // Repeat
            loop {
                token(meta, "test");
                token(meta, ",");
            }
            // End token
            token(meta, "end");
            Ok(())
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
            Token { word: format!("this"), path: path, pos: (0, 0) },
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
