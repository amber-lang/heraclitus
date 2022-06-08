use super::region::Region;

pub struct Rules {
    pub symbols: Vec<char>,
    pub region_tree: Region,
    pub escape_symbol: char
}

impl Rules {
    pub fn new(symbols: Vec<char>, region_tree: Region) -> Rules {
        Rules {
            symbols,
            region_tree,
            escape_symbol: '\\'
        }
    }

    pub fn set_escape(mut self, symbol: char) -> Self {
        self.escape_symbol = symbol;
        self
    }
}