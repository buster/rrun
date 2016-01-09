use runner::Runner;
use execution::execute;

#[derive(Debug)]
pub struct ExternalRunner {
    tpe: String,
    command: String
}


impl ExternalRunner {
    pub fn new(tpe: String, command: String) -> Box<ExternalRunner> {
        return Box::new(ExternalRunner { tpe: tpe, command: command })
    }
}

impl Runner for ExternalRunner {

    fn run(&self, to_run: &str, in_background: bool) -> String {
        //returns a new completion based on the passed string
        let out = execute(self.command.replace("{}", to_run), in_background);
        match out {
            Some(out_string) => out_string.to_owned(),
            None => "".to_string()
        }
    }
    fn get_type(&self) -> String {
        self.tpe.to_string()
    }
}
