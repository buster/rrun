use autocomplete::AutoCompleter;
use execution::execute;

pub struct HistoryAutoCompleter {
    current_history: Vec<String>,
    current_completed_cmd: Option<String>,
    remaining_completions: Vec<String>
}

impl HistoryAutoCompleter {
    pub fn new() -> Box<AutoCompleter> {
        return Box::new(HistoryAutoCompleter {
            current_completed_cmd: None,
            remaining_completions: vec![],
            current_history: vec![]
        })
    }
}

impl AutoCompleter for HistoryAutoCompleter {

    fn complete_next(&mut self) -> Option<String>{
        return self.remaining_completions.pop()
    }

    fn complete_new(&mut self, cmd_string: &str) -> Option<String>{
        let history = execute("history".to_string(), false);
        debug!("cmd string: {}", cmd_string);
        for line in history.unwrap().lines() {
            debug!("found history {}", line);
            if line.starts_with(cmd_string) {

                self.remaining_completions.push(line.to_string());
            }
        }

        self.current_completed_cmd = Some(cmd_string.to_string());
        return self.complete_next()
    }
}


// #[test]
// fn test_history() {
//     let mut new_completion: Box<AutoCompleter> = HistoryAutoCompleter::new();
//     assert!(new_completion.complete_next() != None);
// }
