pub trait AutoCompleter {
    fn get_type(&self) -> String;
    fn complete(&self, cmd_string: &str) -> Box<Iterator<Item = Completion>>;
}

#[derive(Debug,Clone,PartialEq)]
pub struct Completion {
    pub tpe: String,
    pub text: String
}
