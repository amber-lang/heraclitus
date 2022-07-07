use heraclitus::prelude::*;

pub struct Number { value: String }
impl SyntaxModule<SyntaxMetadata> for Number {
    fn new() -> Self {
        Number { value: format!("") }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        self.value = number(meta, vec![])?;
        Ok(())
    }
}