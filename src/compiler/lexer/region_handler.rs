use crate::rules::{Region, Rules};
use super::reader::Reader;

pub struct RegionHandler {
    regions: Vec<Region>,
    escape: char,
    region_stack: Vec<Region>,
}

impl RegionHandler {
    pub fn new(rules: &Rules) -> Self {
        RegionHandler {
            region_stack: vec![],
            regions: rules.regions.clone(),
            escape: rules.escape_symbol
        }
    }

    pub fn get_region(&self) -> Option<&Region> {
        self.region_stack.last()
    }

    // TODO: Make tests for handle region
    pub fn handle_region(&mut self, reader: &Reader) -> bool {
        // If we are not in the global scope
        if let Some(region) = self.get_region() {
            // If current context is lexically preserved
            if region.preserve {
                // Can we create an interpolation here?
                if region.interp.len() > 0 {
                    for interp_name in region.interp.iter() {
                        // Get the region object of this interpolation
                        let interp_region = self.get_region_by_name(interp_name)
                            .expect(format!("Region '{}' does not exists", interp_name).as_str());
                        // The region that got matched based on current code lexing state
                        if let Some(begin_region) = self.match_region_by_begin(reader) {
                            if begin_region.name == *interp_region.name {
                                self.region_stack.push(begin_region);
                                return true;
                            }
                        }
                    }
                }
            }
            // Let's check if we can close current region
            if let Some(end_region) = self.match_region_by_end(reader) {
                if end_region.name == region.name {
                    self.region_stack.pop();
                    return true;
                }
            }
        }
        else {
            // Check if we can open a region
            if let Some(begin_region) = self.match_region_by_begin(reader) {
                self.region_stack.push(begin_region);
                return true;
            }
        }
        false
    }

    // Matches region by some getter callback
    fn match_region_by(&self, reader: &Reader, cb: impl Fn(&Region) -> &String) -> Option<Region> {
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
        self.get_region_by(predicate)
    }

    fn match_region_by_begin(&self, reader: &Reader) -> Option<Region> {
        self.match_region_by(reader, |candidate: &Region| &candidate.begin)
    }

    fn match_region_by_end(&self, reader: &Reader) -> Option<Region> {
        self.match_region_by(reader, |candidate: &Region| &candidate.end)
    }

    fn get_region_by_name(&self, name: &String) -> Option<Region> {
        self.get_region_by(|candidate: &Region| candidate.name == *name)
    }

    fn get_region_by(&self, cb: impl Fn(&Region) -> bool) -> Option<Region> {
        for region in self.regions.iter() {
            if cb(region) {
                return Some(region.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
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
        let code = lines.join("\n");
        let mut reader = super::Reader::new(&code);
        let region = super::Region::new("module-name", "begin", "end");
        let rh = super::RegionHandler {
            region_stack: vec![],
            regions: vec![region],
            escape: '\\'
        };

        let mut result = vec![];
        // Simulate matching regions
        while let Some(_) = reader.next() {
            if let Some(begin) = rh.match_region_by_begin(&reader) {
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
        let mut reader = super::Reader::new(&code);
        let mut rh = super::RegionHandler {
            region_stack: vec![],
            regions: vec![
                super::Region::new("string", "'", "'").add_interp("interpolation"),
                super::Region::new("interpolation", "{", "}").tokenize_inner()
            ],
            escape: '\\'
        };
        let mut result = vec![];
        // Simulate matching regions
        while let Some(_) = reader.next() {
            let region_mutated = rh.handle_region(&reader);
            if region_mutated {
                result.push(reader.get_index());
            }
        }
        assert_eq!(expected, result);      
    }
}
