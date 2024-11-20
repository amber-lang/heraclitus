use crate::compiling_rules::{Region, Rules, RegionMap};
use super::reader::Reader;
use super::reader::ReadMode;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegionReaction {
    Begin(bool),
    End(bool),
    Pass
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegionHandler {
    region_stack: Vec<Region>,
    region_map: RegionMap
}

impl RegionHandler {
    pub fn new(rules: &Rules) -> Self {
        RegionHandler {
            region_stack: vec![rules.region_tree.clone()],
            region_map: rules.region_tree.clone().generate_region_map()
        }
    }

    #[inline]
    pub fn get_region(&self) -> Option<&Region> {
        self.region_stack.last()
    }

    // Error if after code lexing
    // some region was left unclosed
    #[inline]
    pub fn is_region_closed(&self, reader: &Reader) -> Result<(),((usize, usize), Region)> {
        if let Some(region) = self.region_stack.last() {
            if !region.allow_unclosed_region {
                let pos = reader.get_position();
                return Err((pos, region.clone()));
            }
        }
        Ok(())
    }

    // Check where we are in code and open / close some region if matched
    pub fn handle_region(&mut self, reader: &Reader, is_escaped: bool) -> RegionReaction {
        // If we are not in the global scope
        if let Some(region) = self.get_region() {
            for interp_region in region.interp.iter() {
                // The region that got matched based on current code lexing state
                if let Some(mut begin_region) = self.match_region_by_begin(reader, is_escaped) {
                    if begin_region.name == *interp_region.name {
                        // Save the tokenize state here to preserve borrow rules
                        let tokenize = begin_region.tokenize;
                        // This region could reference other region
                        // In this case we want to replace the interpolations
                        // of the region with the target ones
                        if let Some(reference_name) = &begin_region.references {
                            // Try to fetch the region from hash map
                            match self.region_map.get(reference_name) {
                                // If success, then we want to do the replace
                                Some(target_region) => {
                                    begin_region.interp = target_region.interp.clone();
                                },
                                // If fail then it means that we have invalid reference name
                                None => {
                                    panic!("Could not find region with id '{}'", reference_name);
                                }
                            }
                        }
                        self.region_stack.push(begin_region);
                        return RegionReaction::Begin(tokenize);
                    }
                }
            }
            // Let's check if we can close current region
            if let Some(end_region) = self.match_region_by_end(reader, is_escaped) {
                if end_region.name == region.name {
                    // Save the tokenize state here to preserve borrow rules
                    let tokenize = end_region.tokenize;
                    self.region_stack.pop();
                    return RegionReaction::End(tokenize)
                }
            }
        }
        RegionReaction::Pass
    }

    // Matches region by some getter callback
    #[inline]
    fn match_region_by(
        &self,
        reader: &Reader,
        cb: impl Fn(&Region) -> &String,
        candidates: &[Region],
        read_mode: ReadMode,
        is_escaped: bool
    ) -> Option<Region> {
        // Closure that checks if for each given Region is there any that matches current history state
        let predicate = |candidate: &Region| match reader.get_history_or_future(cb(candidate).len(), &read_mode) {
            Some(code_chunk) => !is_escaped && &code_chunk == cb(candidate),
            None => false
        };
        self.get_region_by(predicate, candidates)
    }

    #[inline]
    fn match_region_by_begin(&self, reader: &Reader, is_escaped: bool) -> Option<Region> {
        let region = self.get_region().unwrap();
        self.match_region_by(
            reader,
            |candidate: &Region| &candidate.begin,
            &region.interp,
            ReadMode::Future,
            is_escaped
        )
    }

    #[inline]
    fn match_region_by_end(&self, reader: &Reader, is_escaped: bool) -> Option<Region> {
        let region = self.get_region().unwrap();
        if !region.global {
            self.match_region_by(
                reader,
                |candidate: &Region| &candidate.end,
                &[region.clone()],
                ReadMode::History,
                is_escaped
            )
        } else { None }
    }

    // Target region is a region on which we want to search the interpolations
    #[inline]
    fn get_region_by(&self, cb: impl Fn(&Region) -> bool, candidates: &[Region]) -> Option<Region> {
        for region in candidates.iter() {
            if cb(region) {
                return Some(region.clone())
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::reg;
    use crate::compiling_rules::Region;
    use super::{ RegionHandler, RegionReaction };
    use super::Reader;

    #[test]
    fn match_region() {
        let lines = vec![
            "begin",
            "\\begin",
            "end"
        ];
        let expected = vec![
            (0, String::from("begin")),
            (15, String::from("end"))
        ];
        let code = lines.join(" ");
        let mut reader = Reader::new(&code);
        let region = reg![
            reg!(module as "Module literal" => {
                begin: "begin",
                end: "end"
            })
        ];
        let mut rh = RegionHandler {
            region_stack: vec![region.clone()],
            region_map: region.generate_region_map(),
        };

        let mut result = vec![];
        let mut is_escaped = false;
        // Simulate matching regions
        while let Some(letter) = reader.next() {
            if let Some(begin) = rh.match_region_by_begin(&reader, is_escaped) {
                rh.region_stack.push(begin.clone());
                result.push((reader.get_index(), begin.begin));
            }
            if let Some(end) = rh.match_region_by_end(&reader, is_escaped) {
                result.push((reader.get_index(), end.end));
            }
            // Handle the escape key
            is_escaped = (!is_escaped && letter == '\\')
                .then(|| !is_escaped)
                .unwrap_or(false);
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn handle_region() {
        let lines = vec![
            "'My name is \\\\\\'{name}.\\\\'"
        ];
        let expected = vec![
            0, 16, 21, 25
        ];
        let code = lines.join("\n");
        let region = reg![
            reg!(string as "String literal" => {
                begin: "'",
                end: "'"
            } => [
                reg!(interp as "Interpolation" => {
                    begin: "{",
                    end: "}"
                })
            ])
        ];
        let mut reader = Reader::new(&code);
        let mut rh = RegionHandler {
            region_stack: vec![region.clone()],
            region_map: region.generate_region_map()
        };
        let mut result = vec![];
        let mut is_escaped = false;
        // Simulate matching regions
        while let Some(letter) = reader.next() {
            let region_mutated = rh.handle_region(&reader, is_escaped);
            if let RegionReaction::Begin(_) | RegionReaction::End(_) = region_mutated {
                result.push(reader.get_index());
            }
            // Handle the escape key
            is_escaped = (!is_escaped && letter == '\\')
                .then(|| !is_escaped)
                .unwrap_or(false);
        }
        assert_eq!(expected, result);
    }
}
