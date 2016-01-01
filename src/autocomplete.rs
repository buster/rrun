pub trait AutoCompleter {
    fn complete(&self, cmd_string: &str) -> Box<Iterator<Item = String>>;
}
