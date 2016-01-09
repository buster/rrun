use autocomplete::AutoCompleter;
use autocomplete::Completion;
use runner::Runner;
use externalrunner::ExternalRunner;
use externalautocompleter::ExternalAutoCompleter;
use std::collections::HashMap;
use itertools::Itertools;
use toml;

pub trait Engine {
    fn get_completions(&self, query: &str) -> Box<Iterator<Item=Completion>>;
    fn run_completion(&self, completion: &Completion, in_background: bool) -> Result<String, String>;
}

pub struct DefaultEngine {
    pub completers: Vec<Box<AutoCompleter>>,
    pub runners: HashMap<String, Vec<Box<ExternalRunner>>>
}

impl DefaultEngine {
    pub fn new(config: &toml::Table) -> DefaultEngine {
        DefaultEngine {
            completers: DefaultEngine::get_completers(config),
            runners: DefaultEngine::get_runners(config)
        }
    }
    pub fn get_completers(config: &toml::Table) -> Vec<Box<AutoCompleter>> {
        let maybe_completions = config.get("completion").into_iter();
        let completions = maybe_completions.flat_map(|cs| cs.as_slice().unwrap().into_iter());
        let autocompleter_configs = completions.flat_map(|cs| cs.as_table());
        autocompleter_configs.map(|cfg| {
            let command = cfg.get("command").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap_or("".to_string());
            let tpe = cfg.get("type").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap();
            let trigger = cfg.get("trigger").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap_or("(.*)".to_string());
            ExternalAutoCompleter::new(tpe, command, trigger)
        }).collect()
    }

    pub fn get_runners(config: &toml::Table) -> HashMap<String, Vec<Box<ExternalRunner>>> {
        let runner_configs = config.get("runner").into_iter()
                                  .flat_map(|r| r.as_slice().unwrap().into_iter())
                                  .flat_map(|r| r.as_table());
        let runners: Vec<Box<ExternalRunner>> = runner_configs.map(|cfg| {
            let command = cfg.get("command").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap();
            let tpe = cfg.get("type").and_then(|c| c.as_str()).map(|c| c.to_string()).unwrap();
            ExternalRunner::new(tpe, command)
        }).collect();

        let mut runners_by_type = HashMap::with_capacity(runners.len());
        for (key, group) in runners.into_iter().group_by(|r| r.get_type()) {
            runners_by_type.insert(key, group.into_iter().collect_vec());
        }
        runners_by_type
    }
}

impl Engine for DefaultEngine {
    fn get_completions(&self, query: &str) -> Box<Iterator<Item=Completion>> {
        let completions = self.completers.iter().map(|completer| {
            completer.complete(query).collect_vec().into_iter()
        }).fold1(|c1, c2| c1.chain(c2).collect_vec().into_iter()).unwrap();
        Box::new(completions)
    }

    fn run_completion(&self, completion: &Completion, in_background: bool) -> Result<String, String> {
        let ref runner = self.runners.get(&completion.tpe).unwrap()[0];
        debug!("Running {:?} {:?} with {:?}", completion.tpe, completion, runner);
        Ok(runner.run(&completion.id, in_background))
    }
}
