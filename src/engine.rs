use autocomplete::AutoCompleter;
use autocomplete::Completion;
use runner::Runner;
use externalrunner::ExternalRunner;
use externalautocompleter::ExternalAutoCompleter;
use std::collections::HashMap;
use itertools::Itertools;
use toml;

pub trait Engine {
    fn get_completions(&self, query: &str) -> Box<Iterator<Item = Completion>>;
    fn run_completion(&self, completion: &Completion, in_background: bool) -> Result<String, String>;
}

pub struct DefaultEngine {
    pub completers: Vec<Box<AutoCompleter>>,
    pub runners: HashMap<String, Vec<Box<ExternalRunner>>>,
}

impl DefaultEngine {
    pub fn new(config: &toml::value::Table) -> DefaultEngine {
        DefaultEngine {
            completers: DefaultEngine::get_completers(config),
            runners: DefaultEngine::get_runners(config),
        }
    }
    pub fn get_completers(config: &toml::value::Table) -> Vec<Box<AutoCompleter>> {
        let maybe_completions = config.get("completion").expect("expect completions in config!").as_array().expect("array");
        let mut completes: Vec<Box<AutoCompleter>> = Vec::new();
        debug!("maybe_completions: {:?}", maybe_completions);
        for completion in maybe_completions {
            debug!("completion: {:?}", completion);
            let this_completer = ExternalAutoCompleter::new(
                completion.get("type").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap(),
                completion.get("command").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap_or("".to_string()),
                completion.get("trigger").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap_or("(.*)".to_string())
            );
            completes.push(this_completer);
        }
        completes
    }

    pub fn get_runners(config: &toml::value::Table) -> HashMap<String, Vec<Box<ExternalRunner>>> {
        let runner_configs = config.get("runner").expect("expected runner configs").as_array().expect("array");
        let mut runners: Vec<Box<ExternalRunner>> = Vec::new();

        for runner in runner_configs {
            println!("Runner: {:?}", runner);
            runners.push(ExternalRunner::new(
                runner.get("type").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap(),
                runner.get("command").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap()
            ));
        }
        let mut runners_by_type = HashMap::with_capacity(runners.len());
        for (key, group) in runners.into_iter().group_by(|r| r.get_type()).into_iter() {
            runners_by_type.insert(key, group.into_iter().collect_vec());
        }
        println!("Runners by Type: {:?}", runners_by_type);
        runners_by_type
    }
}

impl Engine for DefaultEngine {
    fn get_completions(&self, query: &str) -> Box<Iterator<Item = Completion>> {
        let completions = self.completers
            .iter()
            .map(|completer| completer.complete(query).collect_vec().into_iter())
            .fold1(|c1, c2| c1.chain(c2).collect_vec().into_iter())
            .unwrap();
        Box::new(completions)
    }

    fn run_completion(&self, completion: &Completion, in_background: bool) -> Result<String, String> {
        let ref runner = match self.runners.get(&completion.tpe) {
            Some(values) if values.len() >= 1 => &values[0],
            Some(_) => return Err("Runner returned zero sized completions".to_owned()),
            None => return Err("Runner returned None".to_owned()),
        };
        debug!("Running {:?} {:?} with {:?}", completion.tpe, completion, runner);
        runner.run(&completion.id, in_background)
    }
}
