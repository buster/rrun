use autocomplete::AutoCompleter;
use autocomplete::Completion;
use execution::execute;
use regex::Regex;

#[derive(Debug)]
pub struct ExternalAutoCompleter {
    tpe: String,
    command: String,
    trigger: String
}


impl ExternalAutoCompleter {
    pub fn new(tpe: String, command: String, trigger: String) -> Box<AutoCompleter> {
        Box::new(ExternalAutoCompleter { tpe: tpe, command: command, trigger: trigger })
    }
}

impl AutoCompleter for ExternalAutoCompleter {

    fn complete(&self, query: &str) -> Box<Iterator<Item=Completion>>{
        let re = Regex::new(&self.trigger).unwrap();
        let trigger_match = re.captures_iter(query).collect::<Vec<_>>();
        let is_applicable = trigger_match.len() > 0;
        debug!("Query {} applicable for {}: {}", query, self.trigger, is_applicable);
        let completion_vec = if is_applicable {
            //returns a new completion based on the passed string
            let query = trigger_match[0].at(1).unwrap_or(query);
            let out = if self.command.len() > 0 {
                execute(self.command.replace("{}", query), false)
            } else {
                Some(query.to_string())
            };
            match out {
                Some(completion_string) => completion_string.lines().map(|l| l.to_owned())
                    .map(|c| {
                        Completion { tpe: self.get_type(), text: c.clone(), id: c.clone() }
                    }).collect::<Vec<_>>(),
                None => vec![]
            }
        } else {
            vec![] as Vec<Completion>
        };
        Box::new(completion_vec.into_iter())
    }
    fn get_type(&self) -> String {
        self.tpe.to_owned()
    }
}

#[test]
fn test_external_completion() {
    let completer = ExternalAutoCompleter::new("command".to_string(), "echo the {}".to_string(), "(.*)".to_string());
    let mut new_completion = completer.complete("foo");
    assert_eq!(new_completion.next(), Some(Completion {tpe: "command".to_owned(), text: "the foo".to_owned(), id: "the foo".to_owned()}));
    assert!(new_completion.next() == None);
    // Test that we didn't break something with mutation
    let mut second_completion = completer.complete("bar");
    assert_eq!(second_completion.next().map(|c| c.text), Some("the bar".to_owned()));
    assert!(second_completion.next() == None);}
