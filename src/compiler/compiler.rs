use std::fs::File;
use std::io::prelude::*;

use crate::rules::Rules;
use crate::compiler::lexer::lexer;

pub struct Compiler {
    pub name: String,
    pub rules: Rules,
    pub code: String,
    pub path: String
}

impl Compiler {
    fn new(name: &str, rules: Rules) -> Self {
        Compiler {
            name: String::from(name),
            rules,
            code: String::new(),
            path: String::from("[code]")
        }
    }

    pub fn load_file(mut self, file_path: String) -> std::io::Result<()> {
        let mut file = File::open(&file_path)?;
        file.read_to_string(&mut self.code)?;
        self.path = file_path;
        Ok(())
    }

    pub fn load(mut self, code: String) {
        self.code = code;
    }

    pub fn set_path(mut self, file_path: String) {
        self.path = file_path;
    }

    pub fn compile(self) -> Self {
        lexer(&self);
        self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let rules = super::Rules {
            symbols: vec![],
            regions: vec![],
            escape_symbol: '\\'
        };
    }
}