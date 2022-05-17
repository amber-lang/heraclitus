use std::fs::File;

use crate::rules::Rules;
use super::lexer;

pub struct Compiler {
    name: String,
    rules: Rules,
    code: String
}

impl Compiler {
    fn new(name: &str, rules: Rules) -> Self {
        Compiler {
            name: String::from(name),
            rules,
            code: String::new()
        }
    }

    pub fn load_file(file: File) {
        
    }

    pub fn load(code: String) {
        
    }

    pub fn compile(mut self) -> Self {
        lexer();
        self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let rules = super::Rules {
            symbols: vec![],
            regions: vec![]   
        };
    }
}