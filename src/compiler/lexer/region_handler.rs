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
            region_stack: Vec::new(),
            regions: rules.regions.clone(),
            escape: rules.escape_symbol
        }
    }

    pub fn get_region(&self) -> Option<&Region> {
        self.region_stack.last()
    }

    pub fn handle_region(&self) {
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
                        // TODO: Finish once Reader is ready
                        
                    }
                    
                    
                }
            }
        }
    }

    // TODO: Make tests for this function

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
        let code = lines.join("\n");
        let mut reader = super::Reader::new(&code);
        let region = super::Region::new("module-name", "begin", "end");
        let rh = super::RegionHandler {
            region_stack: Vec::new(),
            regions: vec![region],
            escape: '\\'
        };
        let left = vec![
            (5, String::from("begin")),
            (16, String::from("end"))
        ];
        let mut right = Vec::new();
        // Simulate matching regions
        while let Some(_) = reader.next() {
            if let Some(begin) = rh.match_region_by_begin(&reader) {
                right.push((reader.index, begin.begin));
            }
            if let Some(end) = rh.match_region_by_end(&reader) {
                right.push((reader.index, end.end));
            }
        }
        assert_eq!(left, right);
    }
}