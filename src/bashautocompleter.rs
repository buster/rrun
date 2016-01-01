use autocomplete::AutoCompleter;
use execution::execute;
pub struct BashAutoCompleter;


impl BashAutoCompleter {
    pub fn new() -> Box<AutoCompleter> {
        return Box::new(BashAutoCompleter)
    }
}

impl AutoCompleter for BashAutoCompleter {

    fn complete(&self, cmd_string: &str) -> Box<Iterator<Item=String>>{
        //returns a new completion based on the passed string
        let out = execute(format!("compgen -A command {}", cmd_string), false);
        let mut completion_vec = match out {
            Some(completion_string) => completion_string.lines().map(|l| l.to_string()).collect::<Vec<_>>(),
            None => vec![]
        };
        completion_vec.reverse();
        Box::new(completion_vec.into_iter())
    }
}

#[test]
fn test_bash_compgen_which() {
    let completer: Box<AutoCompleter> = BashAutoCompleter::new();
    let mut new_completion = completer.complete("which");
    assert_eq!(new_completion.next(), Some("which".to_string()));
    assert!(new_completion.next() != None);
    assert_eq!(completer.complete("Undefined_Command That does not exist").next(), None);
}

#[test]
fn test_bash_compgen_wh() {
    // completing "whic" should complete with "which"
    // if the output of compgen is not reversed (last line is best completion) it may be "while" or
    // something else.
    // this test could fail on different systems where "whic" could complete to something different...
    let completer: Box<AutoCompleter> = BashAutoCompleter::new();
    let mut new_completion = completer.complete("wh");
    assert!(new_completion.next().unwrap().starts_with("whic"));
    assert!(new_completion.next() != None);
    assert_eq!(completer.complete("Undefined_Command That does not exist").next(), None);
}
