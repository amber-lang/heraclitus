use super::region::Region;

pub struct Rules {
    pub symbols: Vec<char>,
    pub regions: Vec<Region>,
    pub escape_symbol: char
}

impl Rules {
    pub fn new(symbols: Vec<char>, regions: Vec<Region>) -> Rules {
        Rules {
            symbols,
            regions,
            escape_symbol: '\\'
        }
    }

    pub fn set_escape(mut self, symbol: char) -> Self {
        self.escape_symbol = symbol;
        self
    }
}