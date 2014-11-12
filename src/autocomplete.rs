use execution;

pub struct BashAutoCompleter {
    current_completed_cmd: Option<String>,
    remaining_completions: Vec<String>
}


impl BashAutoCompleter {

    pub fn new() -> BashAutoCompleter {
        return BashAutoCompleter {
            current_completed_cmd: None,
            remaining_completions: vec![]
        }
    }

    pub fn complete_next(&mut self) -> Option<String>{
        return self.remaining_completions.pop()
    }

    pub fn complete_new(&mut self, cmd_string: &str) -> Option<String>{
        //returns a new completion based on the passed string
        let bash_completions = execution::execute(format!("compgen -A command {}", cmd_string), false);
        //convert return string to vector and set self
        if bash_completions.is_none() { return None };
        for line in bash_completions.unwrap().lines() {
            self.remaining_completions.push(line.into_string());
        }
        self.current_completed_cmd = Some(cmd_string.into_string());
        return self.complete_next()
    }
}

#[test]
fn test_bash_compgen() {
    let mut new_completion = BashAutoCompleter::new();
    assert_eq!(new_completion.complete_new("bash"), Some("bash".into_string()));
    assert!(new_completion.complete_next() != None);
    assert_eq!(new_completion.complete_new("Undefined_Command That does not exist"), None);
}

