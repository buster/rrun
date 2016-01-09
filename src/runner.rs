pub trait Runner {
    fn get_type(&self) -> String;
    fn run(&self, to_run: &str, in_background: bool) -> Result<String, String>;
}
