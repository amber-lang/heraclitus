// TODO: Create syntax modules
type SyntaxModule = ();

#[derive(Clone)]
pub struct Context {
    pub name: String,
    pub begin: String,
    pub end: String,
    pub modules: Vec<SyntaxModule>,
    pub global: bool
}

impl Context {
    pub fn new<T: AsRef<str>>(name: T, begin: T, end: T) -> Context {
        Context {
            name: name.as_ref().to_string(),
            begin: begin.as_ref().to_string(),
            end: end.as_ref().to_string(),
            modules: vec![],
            global: false
        }
    }

    pub fn set_global(mut self) -> Self {
        self.global = true;
        self
    }

    pub fn attach_modules(mut self, modules: Vec<SyntaxModule>) -> Self {
        self.modules = modules;
        self
    }
}