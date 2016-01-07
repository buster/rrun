use autocomplete::AutoCompleter;
use autocomplete::Completion;
use execution::execute;

#[derive(Debug)]
pub struct ExternalAutoCompleter {
    tpe: String,
    command: String
}


impl ExternalAutoCompleter {
    pub fn new(tpe: String, command: String) -> Box<AutoCompleter> {
        Box::new(ExternalAutoCompleter { tpe: tpe, command: command })
    }
}

impl AutoCompleter for ExternalAutoCompleter {

    fn complete(&self, query: &str) -> Box<Iterator<Item=Completion>>{
        //returns a new completion based on the passed string
        let out = execute(self.command.replace("{}", query), false);
        let completion_vec = match out {
            Some(completion_string) => completion_string.lines().map(|l| l.to_owned())
                .map(|c| {
                    Completion { tpe: self.get_type(), text: c }
                }).collect::<Vec<_>>(),
            None => vec![]
        };
        Box::new(completion_vec.into_iter())
    }
    fn get_type(&self) -> String {
        self.tpe.to_owned()
    }
}

#[test]
fn test_external_completion() {
    let completer = ExternalAutoCompleter::new("command".to_string(), "echo the {}".to_string());
    let mut new_completion = completer.complete("foo");
    assert_eq!(new_completion.next(), Some(Completion {tpe: "command".to_owned(), text: "the foo".to_owned()}));
    assert!(new_completion.next() == None);
    // Test that we didn't break something with mutation
    let mut second_completion = completer.complete("bar");
    assert_eq!(second_completion.next().map(|c| c.text), Some("the bar".to_owned()));
    assert!(second_completion.next() == None);}
