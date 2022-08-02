use capitalize::Capitalize;
use std::fs::File;
use std::io::prelude::*;
use crate::rules::Rules;
use crate::compiler::{Token, Lexer, LexerError, LexerErrorType, Metadata, SyntaxModule};
use crate::compiler::logger::{Logger, ErrorDetails};


/// How do you want to separate expressions?
///
/// Separator mode determines how do you want to handle separators (in many languages the semicolon)
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
/// #   fn new() -> Self { GlobalContext {} }
/// #   fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult { Ok(()) }
/// # }
/// # fn compiler() -> Result<(), ErrorDetails> {
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
    /// Rules that describe your language
    pub rules: Rules,
    /// Source code in a form of string
    pub code: String,
    /// Path to the compiled file if exists
    pub path: Option<String>,
    /// Separator mode for this compiler
    pub separator_mode: SeparatorMode,
    /// Scoping mode for this compiler
    pub scoping_mode: ScopingMode,
}

impl Compiler {
    /// Create a new compiler with provided rules of your language
    pub fn new<T: AsRef<str>>(name: T, rules: Rules) -> Self {
        Compiler {
            name: String::from(name.as_ref()),
            rules,
            code: format!(""),
            path: None,
            separator_mode: SeparatorMode::Manual,
            scoping_mode: ScopingMode::Block
        }
    }

    /// Set the language to use indentations
    pub fn use_indents(&mut self) {
        self.scoping_mode = ScopingMode::Indent
    }

    /// Load file from path
    pub fn load_file(mut self, file_path: String) -> std::io::Result<()> {
        let mut file = File::open(&file_path)?;
        file.read_to_string(&mut self.code)?;
        self.path = Some(file_path);
        Ok(())
    }

    /// Load code string
    pub fn load<T: AsRef<str>>(&mut self, code: T) {
        self.code = String::from(code.as_ref());
    }

    /// Set source file path
    pub fn set_path(&mut self, file_path: String) {
        self.path = Some(file_path);
    }

    /// Run just lexer
    pub fn tokenize(&self) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Lexer::new(&self);
        if let Err(data) = lexer.run() {
            return Err(data);
        }
        Ok(lexer.lexem)
    }

    /// Bulk run lexer and parser (used for testing purposes)
    pub fn compile<M: Metadata>(&self, module: &mut impl SyntaxModule<M>) -> Result<M, ErrorDetails> {
        match self.tokenize() {
            Ok(lexem) => {
                let mut meta = M::new(lexem, self.path.clone());
                module.parse(&mut meta)?;
                Ok(meta)
            }
            Err((kind, mut details)) => {
                let data = details.data.clone().unwrap().capitalize();
                // Create an error message
                let message = match kind {
                    LexerErrorType::Singleline => format!("{data} cannot be multiline"),
                    LexerErrorType::Unclosed => format!("{data} unclosed"),
                };
                // Get actual path
                let path = match self.path.clone() {
                    Some(path) => path.clone(),
                    None => format!("[file]")
                };
                // Send error
                Logger::new_err(path, details.get_pos_by_code(&self.code))
                    .attach_message(message)
                    .attach_code(self.code.clone())
                    .show()
                    .exit();
                Err(details)
            }
        }
    }
}
