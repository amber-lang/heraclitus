use capitalize::Capitalize;
use std::fs::File;
use std::io::prelude::*;
use crate::compiling_rules::Rules;
use crate::compiling::{Token, LexerError, LexerErrorType, Metadata, SyntaxModule};
use crate::compiling::failing::message::Message;
use crate::compiling::failing::failure::Failure;
use crate::error_pos;

use super::lexer::Lexer;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// How do you want to separate expressions?
///
/// Separator mode determines how do you want to handle separators (in many languages the semicolon)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SeparatorMode {
    /// Manual separators require user to manually write them all
    Manual,
    /// Compiler uses ASI (automatic semicolon insertion) 
    /// and removes newlines that are not in the context of being a separator.
    /// This variant also requires a string that will represent the separator.
    SemiAutomatic(String),
    /// Compiler instead of inserting semicolons assumes that all newlines end lines.
    /// However user can decide that he wants to continue expression on current line to the next one.
    /// This variant also requires a string that will represent the continuator.
    Automatic(String)
}

/// How do you want to express scopes?
///
/// Scoping mode determines what kind of scoping is used by your language.
/// For instance do you want to use blocks like `{ ... }` or `if ... fi`
/// or do you want to use intents like in languages such as Python or Yaml.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScopingMode {
    /// Scopes are going to be contained between two specified tokens
    Block,
    /// Scopes are going to be determined by the indentation
    Indent
}

/// Compiler that rules them all
/// 
/// Compiler is a central unit of heraclitus.
/// This structure handles tokenizing and parsing considering all your language settings.
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # struct GlobalContext {}
/// # impl SyntaxModule<DefaultMetadata> for GlobalContext {
/// #   syntax_name!("Global");
/// #   fn new() -> Self { GlobalContext {} }
/// #   fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult { Ok(()) }
/// # }
/// # fn compiler() -> Result<(), Failure> {
/// # let rules = Rules::new(vec![], vec![], reg![]);
/// let mut global_ctx = GlobalContext::new();
/// let cc = Compiler::new("HerbScript", rules);
/// let meta = cc.compile(&mut global_ctx)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Compiler {
    /// Name of your language
    pub name: String,
    /// Source code in a form of string
    pub code: Option<String>,
    /// Path to the compiled file if exists
    pub path: Option<String>,
    // Check if user wants to debug parser
    debug: bool,
    /// Lexer to tokenize the code
    lexer: Lexer
}

impl Compiler {
    /// Create a new compiler with provided rules of your language
    pub fn new<T: AsRef<str>>(name: T, rules: Rules) -> Self {
        Compiler {
            name: String::from(name.as_ref()),
            code: None,
            path: None,
            debug: false,
            lexer: Lexer::new(rules)
        }
    }

    /// Set the language to use indentations
    pub fn use_indents(&mut self) {
        self.lexer.scoping_mode = ScopingMode::Indent
    }

    /// Set the language separator mode
    pub fn set_separator(&mut self, mode: SeparatorMode) {
        self.lexer.separator_mode = mode
    }

    /// Load file from path
    pub fn load_file(mut self, file_path: String) -> std::io::Result<()> {
        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.code = Some(contents);
        self.path = Some(file_path);
        Ok(())
    }

    /// Load code string
    pub fn load<T: AsRef<str>>(&mut self, code: T) {
        self.code = Some(String::from(code.as_ref()));
    }

    /// Set source file path
    pub fn set_path(&mut self, file_path: String) {
        self.path = Some(file_path);
    }

    /// Run just lexer
    pub fn tokenize(&self) -> Result<Vec<Token>, LexerError> {
        self.lexer.tokenize(&self.code.clone().unwrap())
    }

    /// Parser will display information about the call stack
    pub fn debug(&mut self) {
        self.debug = true
    }

    /// Bulk run lexer and parser (used for testing purposes)
    pub fn compile<M: Metadata>(&self, module: &mut impl SyntaxModule<M>) -> Result<M, Failure> {
        match self.tokenize() {
            Ok(lexem) => {
                let mut meta = M::new(lexem, self.path.clone(), self.code.clone());
                if self.debug {
                    module.parse_debug(&mut meta)?;
                } else {
                    module.parse(&mut meta)?;
                }
                Ok(meta)
            }
            Err((kind, info)) => {
                let data = info.data.clone().unwrap().capitalize();
                // Create an error message
                let message = match kind {
                    LexerErrorType::Singleline => format!("{data} cannot be multiline"),
                    LexerErrorType::Unclosed => format!("{data} unclosed"),
                };
                // Send error
                let meta = M::new(vec![], self.path.clone(), self.code.clone());
                error_pos!(&meta, info => {
                    message: message,
                    comment: "test"
                })
            }
        }
    }
}
