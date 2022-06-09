use std::collections::HashMap;
pub type RegionMap = HashMap<String,Region>;

#[macro_export]
macro_rules! reg {
    ($id:tt as $name:expr => {begin: $begin:expr, end: $end:expr $(, $option:tt: $value:expr)*} in [$($exp:expr)*]) => ({
        #[allow(unused_mut)]
        let mut region = Region::new(stringify!($id), $name, $begin, $end, vec![$($exp)*], None);
        $(region.$option = $value;)*
        region
    });
    ($id:tt as $name:expr => {begin: $begin:expr, end: $end:expr $(, $option:tt: $value:expr)*}) => ({
        #[allow(unused_mut)]
        let mut region = Region::new(stringify!($id), $name, $begin, $end, vec![], None);
        $(region.$option = $value;)*
        region
    });
    ($id:tt as $name:expr => {begin: $begin:expr, end: $end:expr $(, $option:tt: $value:expr)*} ref $reference:expr) => ({
        #[allow(unused_mut)]
        let mut region = Region::new(stringify!($id), $name, $begin, $end, vec![], Some(stringify!($reference)));
        $(region.$option = $value;)*
        region
    });
    ([$($expr:expr)*]) => (
        Region::new_global(vec![$($expr)*])
    );
}

#[derive(Debug, PartialEq, Clone)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub begin: String,
    pub end: String,
    pub interp: Vec<Region>,
    pub tokenize: bool,
    pub allow_left_open: bool,
    pub global: bool,
    pub references: Option<String>
}

impl Region {
    pub fn new<T: AsRef<str>>(id: T, name: T, begin: T, end: T, interp: Vec<Region>, references: Option<T>) -> Self {
        Region {
            id: String::from(id.as_ref()),
            name: String::from(name.as_ref()),
            begin: String::from(begin.as_ref()),
            end: String::from(end.as_ref()),
            interp,
            // This field determines if the contents
            // of the region should be tokenized
            tokenize: false,
            // This field can allow to leave region 
            // unclosed after parsing has finished
            allow_left_open: false,
            global: false,
            // Region can be a reference to some other region
            references: match references {
                Some(value) => Some(String::from(value.as_ref())),
                None => None
            }
        }
    }

    pub fn new_global(interp: Vec<Region>) -> Region {
        let mut reg = Region::new("global", "Global context", "", "", interp, None);
        reg.allow_left_open = true;
        reg.global = true;
        reg.tokenize = true;
        reg
    }

    // This functionality is required if we want to reference other regions
    pub fn generate_region_map(&self) -> RegionMap {
        pub fn generate_region_rec(this: Region, mut map: RegionMap) -> RegionMap {
            map.insert(this.id.clone(), this.clone());
            for child in this.interp.iter() {
                map = generate_region_rec(child.clone(), map);
            }
            map
        }
        generate_region_rec(self.clone(), HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::{ Region, RegionMap };

    #[test]
    fn region_parses_correctly() {
        let expected = Region {
            id: format!("global"),
            name: format!("Global context"),
            begin: format!(""),
            end: format!(""),
            interp: vec![
                Region {
                    id: format!("string"),
                    name: format!("String Literal"),
                    begin: format!("'"),
                    end: format!("'"),
                    interp: vec![
                        Region {
                            id: format!("string_interp"),
                            name: format!("String Interpolation"),
                            begin: format!("${{"),
                            end: format!("}}"),
                            interp: vec![],
                            tokenize: true,
                            allow_left_open: false,
                            global: false,
                            references: Some(format!("global"))
                        }],
                    tokenize: false,
                    allow_left_open: false,
                    global: false,
                    references: None
                }],
            tokenize: true,
            allow_left_open: true,
            global: true,
            references: None
        };
        let result = reg!([
            reg!(string as "String Literal" => {
                begin: "'",
                end: "'"
            } in [
                reg!(string_interp as "String Interpolation" => {
                    begin: "${",
                    end: "}",
                    tokenize: true
                } ref global)
            ])
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn region_map_correctly() {
        let mut expected: RegionMap = HashMap::new();
        expected.insert("string_interp".to_string(), Region {
            id: "string_interp".to_string(),
            name: "String Interpolation".to_string(),
            begin: "${".to_string(),
            end: "}".to_string(),
            interp: vec![],
            tokenize: true,
            allow_left_open: false,
            global: false,
            references: Some(
                "global".to_string(),
            ),
        });
        expected.insert("global".to_string(), Region {
                id: "global".to_string(),
                name: "Global context".to_string(),
                begin: "".to_string(),
                end: "".to_string(),
                interp: vec![
                    Region {
                        id: "string".to_string(),
                        name: "String Literal".to_string(),
                        begin: "'".to_string(),
                        end: "'".to_string(),
                        interp: vec![
                            Region {
                                id: "string_interp".to_string(),
                                name: "String Interpolation".to_string(),
                                begin: "${".to_string(),
                                end: "}".to_string(),
                                interp: vec![],
                                tokenize: true,
                                allow_left_open: false,
                                global: false,
                                references: Some(
                                    "global".to_string(),
                                ),
                            },
                        ],
                        tokenize: false,
                        allow_left_open: false,
                        global: false,
                        references: None,
                    },
                ],
                tokenize: true,
                allow_left_open: true,
                global: true,
                references: None,
        });
        expected.insert("string".to_string(), Region {
            id: "string".to_string(),
            name: "String Literal".to_string(),
            begin: "'".to_string(),
            end: "'".to_string(),
            interp: vec![
                Region {
                    id: "string_interp".to_string(),
                    name: "String Interpolation".to_string(),
                    begin: "${".to_string(),
                    end: "}".to_string(),
                    interp: vec![],
                    tokenize: true,
                    allow_left_open: false,
                    global: false,
                    references: Some(
                        "global".to_string(),
                    ),
                },
            ],
            tokenize: false,
            allow_left_open: false,
            global: false,
            references: None,
        });
        let region = reg!([
            reg!(string as "String Literal" => {
                begin: "'",
                end: "'"
            } in [
                reg!(string_interp as "String Interpolation" => {
                    begin: "${",
                    end: "}",
                    tokenize: true
                } ref global)
            ])
        ]);
        let result = region.generate_region_map();
        assert_eq!(expected, result);
    }
}