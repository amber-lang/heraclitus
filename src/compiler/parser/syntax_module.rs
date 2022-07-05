use crate::compiler::Token;

pub struct SyntaxMetadata {
    pub index: usize,
    pub expr: Vec<Token>
}

impl SyntaxMetadata {
    pub fn new(expression: Vec<Token>) -> Self {
        SyntaxMetadata {
            index: 0,
            expr: expression
        }
    }
}

impl Metadata for SyntaxMetadata {
    fn get_token_at(&self, index: usize) -> Option<Token> {
        if let Some(token) = self.expr.get(index) {
            return Some(token.clone())
        } else { None }
    }
    fn set_index(&mut self, index: usize) {
        self.index = index
    }
    fn get_index(&self) -> usize {
        self.index
    }
}

pub trait Metadata {
    fn get_token_at(&self, index: usize) -> Option<Token>;
    fn get_index(&self) -> usize;
    fn set_index(&mut self, index: usize);
}

pub type SyntaxResult = Result<(),()>;

pub trait SyntaxModule<M> {
    fn new() -> Self;
    fn parse(&mut self, meta: &mut M) -> SyntaxResult;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::parser::pattern::*;
    use crate::compiler::parser::preset::*;

    struct Expression {}
    impl SyntaxModule<SyntaxMetadata> for Expression {
        fn new() -> Self {
            Expression {  }
        }
        fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
            token(meta, "let")?;
            Ok(())
        }
    }

    #[test]
    fn test_token_match() {
        let mut exp = Expression {};
        let dataset1 = vec![
            Token {
                word: format!("let"),
                pos: (0, 0)
            }
        ];
        let dataset2 = vec![
            Token {
                word: format!("tell"),
                pos: (0, 0)
            }
        ];
        let result1 = exp.parse(&mut SyntaxMetadata::new(&dataset1));
        let result2 = exp.parse(&mut SyntaxMetadata::new(&dataset2));
        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    struct Preset {}
    impl SyntaxModule<SyntaxMetadata> for Preset {
        fn new() -> Self {
            Preset {  }
        }
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
            Token { word: format!("_text"), pos: (0, 0) },
            // Numeric
            Token { word: format!("12321"), pos: (0, 0) },
            // Number
            Token { word: format!("-123.12"), pos: (0, 0) },
            // Integer
            Token { word: format!("-12"), pos: (0, 0) },
            // Float
            Token { word: format!("-.681"), pos: (0, 0)}
        ];
        let result = exp.parse(&mut SyntaxMetadata::new(&dataset));
        assert!(result.is_ok());
    }

    struct PatternModule {}
    impl SyntaxModule<SyntaxMetadata> for PatternModule {
        fn new() -> Self {
            PatternModule {  }
        }
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
            syntax(meta, &mut Expression {})?;
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
            Token { word: format!("orange"), pos: (0, 0) },
            Token { word: format!("optional"), pos: (0, 0) },
            Token { word: format!("let"), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!(","), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!("end"), pos: (0, 0) }
        ];
        // Token should fail
        let dataset2 = vec![
            Token { word: format!("kiwi"), pos: (0, 0) },
            Token { word: format!("optional"), pos: (0, 0) },
            Token { word: format!("let"), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!(","), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!("end"), pos: (0, 0) }
        ];
        // Syntax should fail
        let dataset3 = vec![
            Token { word: format!("orange"), pos: (0, 0) },
            Token { word: format!("tell"), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!(","), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!("end"), pos: (0, 0) }
        ];
        // Token should fail because of repeat matching (this , this) ,
        let dataset4 = vec![
            Token { word: format!("orange"), pos: (0, 0) },
            Token { word: format!("tell"), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!(","), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!("this"), pos: (0, 0) },
            Token { word: format!("end"), pos: (0, 0) }
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
