use crate::rules::{Region, Rules};
use super::reader::Reader;

pub struct RegionHandler<'a> {
    regions: Vec<Region>,
    escape: char,
    region_stack: Vec<Region>,
    reader: &'a Reader<'a>
}

impl<'a> RegionHandler<'a> {
    pub fn new(rules: &Rules, reader: &'a Reader) -> Self {
        RegionHandler {
            region_stack: Vec::new(),
            regions: rules.regions.clone(),
            escape: rules.escape_symbol,
            reader
        }
    }

    pub fn get_region(&'a self) -> Option<&'a Region> {
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
    fn match_region_by(&self, cb: impl Fn(&Region) -> &String) -> Option<Region> {
        // Closure that checks if for each given Region is there any that matches current history state
        let predicate = |candidate: &Region| match self.reader.get_history(cb(candidate).len()) {
            Some(code_chunk) => {
                // Check if the region was escaped
                let is_escaped = match self.reader.get_history(cb(candidate).len() + 1) {
                    Some(code_chunk_with_escape) => code_chunk_with_escape.chars().next().unwrap() == self.escape,
                    None => false
                };
                !is_escaped && code_chunk == cb(candidate)
            }
            None => false
        };
        self.get_region_by(predicate)
    }

    fn match_region_by_begin(&self) -> Option<Region> {
        self.match_region_by(|candidate: &Region| &candidate.name)
    }

    fn match_region_by_end(&self) -> Option<Region> {
        self.match_region_by(|candidate: &Region| &candidate.end)
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
            "start",
            "some code",
            "end"
        ];
        let code = lines.join("\n");
        let mut reader = super::Reader::new(&code);
        let region = super::Region::new("module-name", "start", "end");
        let rh = super::RegionHandler {
            region_stack: Vec::new(),
            regions: vec![region],
            escape: '\\',
            reader: &reader
        };
        // Simulate reading
        for _ in 0..lines[0].len() {
            // TODO: Fix this issue
            reader.next_letter();
        }
        let region = rh.match_region_by_begin();
        println!("{:?}", region);
    }
}