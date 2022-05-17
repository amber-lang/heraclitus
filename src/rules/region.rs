#[derive(Debug, PartialEq)]
pub struct Region {
    name: String,
    start: String,
    end: String,
    interp: Vec<String>,
    preserve: bool
}

impl Region {
    pub fn new(name: &str, start: &str, end: &str) -> Self {
        Region {
            name: String::from(name),
            start: String::from(start),
            end: String::from(end),
            interp: vec![],
            preserve: true
        }
    }

    pub fn add_interp(mut self, name: &str) -> Self {
        self.interp.push(String::from(name));
        self
    }

    pub fn tokenize_inner(mut self) -> Self {
        self.preserve = false;
        self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn region_parses_correctly() {
        // Sloppy Region initialization
        let left = super::Region {
            name: String::from("module-name"),
            start: String::from("{"),
            end: String::from("}"),
            interp: vec![
                String::from("sub-module-1"),
                String::from("sub-module-2"),
                String::from("sub-module-3")
            ],
            preserve: false
        };
        // Clean Region initialization using struct implementation
        let right = super::Region::new("module-name", "{", "}")
            .add_interp("sub-module-1")
            .add_interp("sub-module-2")
            .add_interp("sub-module-3")
            .tokenize_inner();
        assert_eq!(left, right);
    }
}