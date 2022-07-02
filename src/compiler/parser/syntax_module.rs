use crate::compiler::Token;

pub struct SyntaxMetadata<'a> {
    pub index: usize,
    pub is_indent: bool,
    pub indent_level: usize,
    pub indent_stack: Vec<usize>,
    pub expr: &'a [Token<'a>]
}

impl<'a> SyntaxMetadata<'a> {
    pub fn new(expression: &'a [Token]) -> Self {
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
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::parser::pattern::*;
    use crate::compiler::parser::preset::*;

    struct Expression {}
    impl SyntaxModule for Expression {
        fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
            token(meta, "let")?;
            Ok(())
        }
    }

    #[test]
    fn test_token_match() {
        let mut exp = Expression {};
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
        fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
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
        let mut exp = Preset {};
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

    struct PatternModule {}
    impl SyntaxModule for PatternModule {
        #[allow(unused_must_use)]
        fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
            // Any
            if let Ok(_) = token(meta, "apple") {}
            else if let Ok(_) = token(meta, "orange") {}
            else if let Ok(_) = token(meta, "banana") {}
            else { return Err(()) }
            // Optional
            token(meta, "optional");
            // Syntax
            syntax(meta, Box::new(Expression {}))?;
            // Repeat
            loop {
                if let Err(()) = token(meta, "test") {
                    break;
                }
                if let Err(()) = token(meta, ",") {
                    break;
                }
            }
            // End token
            token(meta, "end");
            Ok(())
        }
    }

    #[test]
    fn rest_match() {
        let mut exp = PatternModule {};
        let path = &format!("/path/to/file");
        // Everything should pass
        let dataset1 = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("optional"), path: path, pos: (0, 0) },
            Token { word: format!("let"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
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
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        // Syntax should fail
        let dataset3 = vec![
            Token { word: format!("orange"), path: path, pos: (0, 0) },
            Token { word: format!("tell"), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
            Token { word: format!(","), path: path, pos: (0, 0) },
            Token { word: format!("this"), path: path, pos: (0, 0) },
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
            Token { word: format!("end"), path: path, pos: (0, 0) }
        ];
        let result1 = exp.parse(&mut SyntaxMetadata::new(&dataset1));
        let result2 = exp.parse(&mut SyntaxMetadata::new(&dataset2));
        let result3 = exp.parse(&mut SyntaxMetadata::new(&dataset3));
        let result4 = exp.parse(&mut SyntaxMetadata::new(&dataset4));
        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());
    }

}
