#[derive(Debug, PartialEq, Clone)]
pub struct GlobalRegion {
    pub interp: Vec<String>
}

// TODO: Modify the "Region" struct
#[derive(Debug, PartialEq, Clone)]
pub struct Region {
    pub name: String,
    pub begin: String,
    pub end: String,
    pub interp: Vec<String>,
    pub preserve: bool,
    pub allow_left_open: bool
}

// By default all options are set to 'false'
// If you want to set a change - set one to 'true'
#[derive(Debug, PartialEq, Clone)]
pub struct CustomRegion {
    pub name: String,
    pub begin: String,
    pub end: String,
    pub interp: Vec<String>,
    pub tokenize: bool,
    pub allow_left_open: bool
}

impl Region {
    pub fn new<T: AsRef<str>>(name: T, begin: T, end: T) -> Self {
        Region {
            name: String::from(name.as_ref()),
            begin: String::from(begin.as_ref()),
            end: String::from(end.as_ref()),
            interp: vec![],
            // This field determines if the contents
            // of the region should be tokenized
            preserve: true,
            // This field can allow to leave region 
            // unclosed after parsing has finished
            allow_left_open: false
        }
    }

    pub fn add_interp<T: AsRef<str>>(mut self, name: T) -> Self {
        self.interp.push(String::from(name.as_ref()));
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