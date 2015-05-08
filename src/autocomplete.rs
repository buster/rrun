pub trait AutoCompleter {
    fn complete_next(&mut self) -> Option<String>;
    fn complete_new(&mut self, cmd_string: &str) -> Option<String>;
}
