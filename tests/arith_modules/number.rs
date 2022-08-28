use heraclitus_compiler::prelude::*;

#[derive(Debug)]
pub struct Number {
    value: String
}

impl SyntaxModule<DefaultMetadata> for Number {
    syntax_name!("Number");
    fn new() -> Self {
        Number { value: String::new() }
    }
    fn parse(&mut self, meta: &mut DefaultMetadata) -> SyntaxResult {
        self.value = number(meta, vec![])?;
        Ok(())
    }
}