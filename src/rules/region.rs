#[derive(Debug, PartialEq, Clone)]
pub struct Region {
    pub name: String,
    pub begin: String,
    pub end: String,
    pub interp: Vec<String>,
    pub preserve: bool,
    pub allow_left_open: bool
}

impl Region {
    pub fn new(name: &str, begin: &str, end: &str) -> Self {
        Region {
            name: String::from(name),
            begin: String::from(begin),
            end: String::from(end),
            interp: vec![],
            // This field determines if the contents
            // of the region should be tokenized
            preserve: true,
            // This field can allow to leave region 
            // unclosed after parsing has finished
            allow_left_open: false
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
            name: String::from("Array literal"),
            begin: String::from("["),
            end: String::from("]"),
            interp: vec![
                String::from("Sub module 1"),
                String::from("Sub module 2"),
                String::from("Sub module 3")
            ],
            preserve: false,
            allow_left_open: false
        };
        // Clean Region initialization using struct implementation
        let right = super::Region::new("Array literal", "[", "]")
            .add_interp("Sub module 1")
            .add_interp("Sub module 2")
            .add_interp("Sub module 3")
            .tokenize_inner();
        assert_eq!(left, right);
    }
}
