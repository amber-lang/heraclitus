use std::collections::HashMap;
use crate::compiling_rules::Rules;
use super::reader::Reader;

#[derive(Debug, PartialEq, Eq)]
pub enum CompoundReaction {
    Begin,
    Keep,
    End,
    Pass
}

pub struct CompoundHandler {
    compound_tree: HashMap<char, Vec<char>>,
    is_triggered: bool
}

// Handles compounds
impl CompoundHandler {
    pub fn new(rules: &Rules) -> Self {
        CompoundHandler {
            compound_tree: Self::generate_compunds(rules.compounds.clone()),
            is_triggered: false
        }
    }

    // Generates a tree where the key is the left item of 
    // the pair and values are all the right items of the pair
    fn generate_compunds(word_pairs: Vec<(char, char)>) -> HashMap<char, Vec<char>> {
        let mut compound_tree = HashMap::new();
        for (left, right) in word_pairs {
            compound_tree
                .entry(left)
                .or_insert(vec![])
                .push(right);
        }
        compound_tree
    }

    // Determines what shall we do with detected compound
    pub fn handle_compound(&mut self, letter: char, reader: &Reader, is_tokenize: bool) -> CompoundReaction {
        // If the region is not tokenizable, we do not need to check for compounds
        if !is_tokenize {
            self.is_triggered = false;
            return CompoundReaction::Pass;
        }
        // Get completing symbol for current symbol
        if let Some(entries) = self.compound_tree.get(&letter) {
            // For any of the completing symbols
            // check if future symbol satisfies at least one
            for entry in entries.iter() {
                // Get future string of current letter and the next one
                if let Some(future) = reader.get_future(2) {
                    let future_letter = future.chars().nth(1).unwrap();
                    // Check if next character matches our desired symbol
                    if future_letter == *entry {
                        // If we matched before as well then this means
                        // that this is a chain of compounds
                        return if self.is_triggered {
                            CompoundReaction::Keep
                        }
                        // If it's the first time, then this means
                        // that this is a beginning
                        else {
                            self.is_triggered = true;
                            CompoundReaction::Begin
                        }
                    }
                }
            }
        }
        // If we matched before and no match
        // was found - end the compound
        if self.is_triggered {
            self.is_triggered = false;
            CompoundReaction::End
        }
        // If nothing happened and we didn't
        // match before - carry on with lexing
        else {
            CompoundReaction::Pass
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;
    use crate::reg;
    use crate::compiling_rules::Region;
    use crate::compiling_rules::Rules;
    use super::{CompoundHandler, CompoundReaction};
    use super::Reader;

    #[test]
    fn match_region() {
        let expected = vec![
            CompoundReaction::Pass,
            CompoundReaction::Begin,
            CompoundReaction::Keep,
            CompoundReaction::End,
            CompoundReaction::Begin,
            CompoundReaction::End,
        ];
        let code = format!("!<=><=");
        let mut reader = Reader::new(&code);
        let symbols = vec!['<', '=', '>'];
        let compounds = vec![('<', '='), ('=', '>')];
        let rules = Rules::new(symbols, compounds, reg![]);
        let mut ch = CompoundHandler::new(&rules);
        let mut result = vec![];
        // Simulate matching compounds
        while let Some(letter) = reader.next() {
            result.push(ch.handle_compound(letter, &reader, true));
        }
        assert_eq!(expected, result);
    }
}