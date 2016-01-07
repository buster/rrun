use autocomplete::AutoCompleter;
use execution::execute;

pub struct ExternalAutoCompleter {
    command: String
}


impl ExternalAutoCompleter {
    pub fn new(command: String) -> Box<AutoCompleter> {
        return Box::new(ExternalAutoCompleter { command: command })
    }
}

impl AutoCompleter for ExternalAutoCompleter {

    fn complete(&self, query: &str) -> Box<Iterator<Item=String>>{
        //returns a new completion based on the passed string
        let out = execute(self.command.replace("{}", query), false);
        let completion_vec = match out {
            Some(completion_string) => completion_string.lines().map(|l| l.to_owned()).collect::<Vec<_>>(),
            None => vec![]
        };
        Box::new(completion_vec.into_iter())
    }
}

#[test]
fn test_external_completion() {
    let completer = ExternalAutoCompleter::new("echo the {}".to_string());
    let mut new_completion = completer.complete("foo");
    assert_eq!(new_completion.next(), Some("the foo".to_owned()));
    assert!(new_completion.next() == None);
    // Test that we didn't break something with mutation
    let mut second_completion = completer.complete("bar");
    assert_eq!(second_completion.next(), Some("the bar".to_owned()));
    assert!(second_completion.next() == None);}
