use crate::compiling::failing::failure::Failure;
use colored::Colorize;

use super::Metadata;

#[macro_export]
/// This macro is a syntax sugar for the name method
/// All this does is setting the new name without all the syntax clutter.
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # struct MySyntax;
/// impl SyntaxModule<DefaultMetadata> for MySyntax {
///     syntax_name!("MySyntax");
/// #   fn new() -> Self { Self {} }
/// #   fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult { Ok(()) }
///     // ...
/// }
/// ```
macro_rules! syntax_name {
    ($expr:expr) => {
        fn name() -> &'static str {
            $expr
        }
    };
}

/// Result that should be returned in the parsing phase
pub type SyntaxResult = Result<(), Failure>;

/// Trait for parsing
///
/// Trait that should be implemented in order to parse tokens with heraklit
/// ```
/// # use heraclitus_compiler::prelude::*;
/// struct MySyntax {
///     name: String
///     // ... (you decide what you need to store)
/// }
///
/// impl SyntaxModule<DefaultMetadata> for MySyntax {
///     syntax_name!("MySyntax");
///
///     fn new() -> MySyntax {
///         MySyntax {
///             name: format!(""),
///             // Default initialization
///         }
///     }
///
///     // Here you can parse the actual code
///     fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
///         token(meta, "var")?;
///         self.name = variable(meta, vec!['_'])?;
///         Ok(())
///     }
/// }
/// ```
pub trait SyntaxModule<M: Metadata> {
    /// Create a new default implementation of syntax module
    fn new() -> Self;
    /// Name of this module
    fn name() -> &'static str;
    /// Parse and create AST
    ///
    /// This method is fundamental in creating a functional AST node that can determine
    /// if tokens provided by metadata can be consumed to create this particular AST node.
    fn parse(&mut self, meta: &mut M) -> SyntaxResult;
    /// Do not implement this function as this is a predefined function for debugging
    fn parse_debug(&mut self, meta: &mut M) -> SyntaxResult {
        match meta.get_debug() {
            Some(debug) => {
                let padding = "  ".repeat(debug);
                println!("{padding}[Entered] {}", Self::name());
                meta.set_debug(debug + 1);
                let time = std::time::Instant::now();
                let result = self.parse(meta);
                match result {
                    Ok(()) => println!("{padding}{} {} ({}ms)", "[Left]".green(), Self::name(), time.elapsed().as_millis()),
                    Err(_) => println!("{padding}{} {} ({}ms)", "[Failed]".red(), Self::name(), time.elapsed().as_millis())
                }
                meta.set_debug(debug);
                result
            }
            None => {
                meta.set_debug(0);
                self.parse_debug(meta)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiling::parser::pattern::*;
    use crate::compiling::parser::preset::*;
    use crate::compiling::{ Token, DefaultMetadata };

    struct Expression {}
    impl SyntaxModule<DefaultMetadata> for Expression {
        syntax_name!("Expression");

        fn new() -> Self {
            Expression { }
        }
        fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
            token(meta, "let")?;
            Ok(())
        }
    }

    #[test]
    fn test_token_match() {
        let mut exp = Expression {};
        let dataset1 = vec![
            Token {
                word: "let".to_string(),
                pos: (0, 0),
                start: 0
            }
        ];
        let dataset2 = vec![
            Token {
                word: "tell".to_string(),
                pos: (0, 0),
                start: 0
            }
        ];
        let result1 = exp.parse(&mut DefaultMetadata::new(dataset1, ""));
        let result2 = exp.parse(&mut DefaultMetadata::new(dataset2, ""));
        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    struct Preset {}
    impl SyntaxModule<DefaultMetadata> for Preset {
        syntax_name!("Preset");
        fn new() -> Self {
            Preset {  }
        }
        fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
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
        let dataset = vec![
            // Variable
            Token { word: "_text".to_string(), pos: (0, 0), start: 0 },
            // Numeric
            Token { word: "12321".to_string(), pos: (0, 0), start: 0 },
            // Number
            Token { word: "-123.12".to_string(), pos: (0, 0), start: 0 },
            // Integer
            Token { word: "-12".to_string(), pos: (0, 0), start: 0 },
            // Float
            Token { word: "-.681".to_string(), pos: (0, 0), start: 0 }
        ];
        let result = exp.parse(&mut DefaultMetadata::new(dataset, ""));
        assert!(result.is_ok());
    }

    struct PatternModule {}
    impl SyntaxModule<DefaultMetadata> for PatternModule {
        syntax_name!("Pattern Module");
        fn new() -> Self {
            PatternModule {  }
        }
        #[allow(unused_must_use)]
        fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
            // Any
            if token(meta, "apple").is_err() && token(meta, "orange").is_err() {
                token(meta, "banana")?;
            }
            // Optional
            token(meta, "optional");
            // Syntax
            syntax(meta, &mut Expression::new())?;
            // Repeat
            loop {
                if token(meta, "test").is_err() {
                    break;
                }
                if token(meta, ",").is_err() {
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
        // Everything should pass
        let dataset1 = vec![
            Token { word: "orange".to_string(), pos: (0, 0), start: 0 },
            Token { word: "optional".to_string(), pos: (0, 0), start: 0 },
            Token { word: "let".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: ",".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: "end".to_string(), pos: (0, 0), start: 0 }
        ];
        // Token should fail
        let dataset2 = vec![
            Token { word: "kiwi".to_string(), pos: (0, 0), start: 0 },
            Token { word: "optional".to_string(), pos: (0, 0), start: 0 },
            Token { word: "let".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: ",".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: "end".to_string(), pos: (0, 0), start: 0 }
        ];
        // Syntax should fail
        let dataset3 = vec![
            Token { word: "orange".to_string(), pos: (0, 0), start: 0 },
            Token { word: "tell".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: ",".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: "end".to_string(), pos: (0, 0), start: 0 }
        ];
        // Token should fail because of repeat matching (this , this) ,
        let dataset4 = vec![
            Token { word: "orange".to_string(), pos: (0, 0), start: 0 },
            Token { word: "tell".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: ",".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: "this".to_string(), pos: (0, 0), start: 0 },
            Token { word: "end".to_string(), pos: (0, 0), start: 0 }
        ];
        let result1 = exp.parse(&mut DefaultMetadata::new(dataset1, ""));
        let result2 = exp.parse(&mut DefaultMetadata::new(dataset2, ""));
        let result3 = exp.parse(&mut DefaultMetadata::new(dataset3, ""));
        let result4 = exp.parse(&mut DefaultMetadata::new(dataset4, ""));
        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());
    }

}
