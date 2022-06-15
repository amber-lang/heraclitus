use super::region::Region;
use super::context::Context;

pub struct Rules {
    pub symbols: Vec<char>,
    pub region_tree: Region,
    pub escape_symbol: char,
    pub scopes: Vec<Context>,
    pub global_scope: Option<Context>
}

impl Rules {
    pub fn new(symbols: Vec<char>, region_tree: Region) -> Rules {
        Rules {
            symbols,
            region_tree,
            escape_symbol: '\\',
            scopes: vec![],
            global_scope: None
        }
    }

    pub fn attach_scopes(mut self, scopes: Vec<Context>) -> Self {
        self.scopes = scopes;
        match self.scopes.iter().find(|item| item.global) {
            Some(global) => self.global_scope = Some(global.clone()),
            None => panic!("Could not find global context")
        }
        self
    }

    pub fn set_escape(mut self, symbol: char) -> Self {
        self.escape_symbol = symbol;
        self
    }
}