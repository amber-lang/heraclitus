use capitalize::Capitalize;
use std::fs::File;
use std::io::prelude::*;
use crate::{ Token, Rules, Lexer, SyntaxModule, Metadata, LexerErrorType, LexerError };
use crate::logger::{Log, ErrorDetails};

#[derive(Clone, PartialEq)]
pub enum SeparatorMode {
    Manual,
    SemiAutomatic,
    Automatic
}

#[derive(Clone, PartialEq)]
pub enum ScopingMode {
    Block,
    Indent
}

pub struct Compiler {
    pub name: String,
    pub rules: Rules,
    pub code: String,
    pub path: Option<String>,
    pub separator_mode: SeparatorMode,
    pub scoping_mode: ScopingMode,
}

impl Compiler {
    pub fn new(name: &str, rules: Rules) -> Self {
        Compiler {
            name: String::from(name),
            rules,
            code: format!(""),
            path: None,
            separator_mode: SeparatorMode::Automatic,
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
                Log::new_err(path, details.get_pos_by_code(&self.code))
                    .attach_message(message)
                    .attach_code(self.code.clone())
                    .show()
                    .exit();
                Err(details)
            }
        }
    }
}
