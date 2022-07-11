use crate::rules::{Region, Rules, RegionMap};
use super::super::logger::Log;
use super::reader::Reader;

#[derive(PartialEq)]
pub enum Reaction {
    Begin,
    End,
    Pass
}

pub struct RegionHandler {
    region_stack: Vec<Region>,
    region_map: RegionMap,
    escape: char
}

impl RegionHandler {
    pub fn new(rules: &Rules) -> Self {
        RegionHandler {
            region_stack: vec![rules.region_tree.clone()],
            region_map: rules.region_tree.clone().generate_region_map(),
            escape: rules.escape_symbol
        }
    }

    pub fn get_region(&self) -> Option<&Region> {
        self.region_stack.last()
    }

    // Error if after code lexing 
    // some region was left unclosed
    pub fn is_region_closed(&self, reader: &Reader) -> Result<(),(usize, usize, Region)> {
        if let Some(region) = self.region_stack.last() {
            if !region.allow_left_open {
                let (row, col) = reader.get_position();
                return Err((row, col, region.clone()));
                // Log::new_err(path, row, col)
                //     .attach_message(format!("Unclosed {}", region.name))
                //     .attach_code(reader.code)
                //     .send()
                //     .exit();
            }
        }
        Ok(())
    }

    // Check where we are in code and open / close some region if matched
    pub fn handle_region(&mut self, reader: &Reader) -> Reaction {
        // If we are not in the global scope
        if let Some(region) = self.get_region() {
            for interp_region in region.interp.iter() {
                // The region that got matched based on current code lexing state
                if let Some(mut begin_region) = self.match_region_by_begin(reader) {
                    if begin_region.name == *interp_region.name {
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
                        return Reaction::Begin
                    }
                }
            }
            // Let's check if we can close current region
            if let Some(end_region) = self.match_region_by_end(reader) {
                if end_region.name == region.name {
                    self.region_stack.pop();
                    return Reaction::End
                }
            }
        }
        Reaction::Pass
    }

    // Matches region by some getter callback
    fn match_region_by(&self, reader: &Reader, cb: impl Fn(&Region) -> &String, candidates: &Vec<Region>) -> Option<Region> {
        // Closure that checks if for each given Region is there any that matches current history state
        let predicate = |candidate: &Region| match reader.get_history(cb(candidate).len()) {
            Some(code_chunk) => {
                // Check if the region was escaped
                let is_escaped = match reader.get_history(cb(candidate).len() + 1) {
                    Some(code_chunk_with_escape) => code_chunk_with_escape.chars().next().unwrap() == self.escape,
                    None => false
                };
                !is_escaped && code_chunk == cb(candidate)
            }
            None => false
        };
        self.get_region_by(predicate, candidates)
    }

    fn match_region_by_begin(&self, reader: &Reader) -> Option<Region> {
        let region = self.get_region().unwrap();
        self.match_region_by(reader, |candidate: &Region| &candidate.begin, &region.interp)
    }

    fn match_region_by_end(&self, reader: &Reader) -> Option<Region> {
        let region = self.get_region().unwrap();
        if !region.global {
            self.match_region_by(reader, |candidate: &Region| &candidate.end, &vec![region.clone()])
        } else { None }
    }

    // Target region is a region on which we want to search the interpolations
    fn get_region_by(&self, cb: impl Fn(&Region) -> bool, candidates: &Vec<Region>) -> Option<Region> {
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
    use crate::rules::Region;
    use super::{ RegionHandler, Reaction };
    use super::Reader;

    #[test]
    fn match_region() {
        let lines = vec![
            "begin",
            "\\begin",
            "end"
        ];
        let expected = vec![
            (4, String::from("begin")),
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
            escape: '\\'
        };

        let mut result = vec![];
        // Simulate matching regions
        while let Some(_) = reader.next() {
            if let Some(begin) = rh.match_region_by_begin(&reader) {
                rh.region_stack.push(begin.clone());
                result.push((reader.get_index(), begin.begin));
            }
            if let Some(end) = rh.match_region_by_end(&reader) {
                result.push((reader.get_index(), end.end));
            }
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn handle_region() {
        let lines = vec![
            "'My name is {name}.'"
        ];
        let expected = vec![
            0, 12, 17, 19
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
            region_map: region.generate_region_map(),
            escape: '\\'
        };
        let mut result = vec![];
        // Simulate matching regions
        while let Some(_) = reader.next() {
            let region_mutated = rh.handle_region(&reader);
            if let Reaction::Begin | Reaction::End = region_mutated {
                result.push(reader.get_index());
            }
        }
        assert_eq!(expected, result);
    }
}
