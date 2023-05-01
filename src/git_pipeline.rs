use crate::get_rules;
use crate::Fragment;
use crate::Leak;
use crate::Logs;
use crate::MatchPipeline;
use crate::Rule;
use rayon::prelude::*;
use std::collections::hash_map::HashMap;
use std::sync::{Arc, Mutex};

pub struct GitPipeline {
    path: String,
    fragments: Vec<Fragment>,
    rules: Vec<Rule>,
    pub data: Vec<Leak>,
}

impl GitPipeline {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            fragments: Vec::new(),
            rules: get_rules(),
            data: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        return self.run();
    }

    fn run(&mut self) {
        self.get_logs();
        return self.search_leaks();
    }

    fn get_logs(&mut self) {
        let mut log_obj = Logs::new(&self.path);
        log_obj.start();
        self.fragments.extend(log_obj.logs);
    }

    fn search_leaks(&mut self) {
        let shared_leaks: HashMap<String, Leak> = HashMap::new();
        let shared_data = Arc::new(Mutex::new(shared_leaks));
        self.fragments.par_iter().for_each(|fragment| {
            self.rules.par_iter().for_each(|rule| {
                rule.keywords.par_iter().for_each(|keyword| {
                    if fragment.body.to_lowercase().contains(keyword) {
                        let mut match_obj = MatchPipeline::new(true);
                        match_obj.search(rule, fragment);

                        for leak in match_obj.leaks {
                            let temp_leaks = shared_data.lock();
                            match temp_leaks {
                                Ok(mut temp_leaks) => {
                                    let secret = leak.secret.trim().to_string();
                                    let key = format!("{}_{}_{}", secret, leak.file, leak.line);
                                    temp_leaks.insert(key, leak);
                                }
                                Err(_) => {
                                    println!("Could Not Store {:#?}", leak);
                                }
                            }
                        }
                    }
                });
            });
        });

        let all_leaks = shared_data.lock();
        match all_leaks {
            Ok(all_leaks) => {
                for leak in all_leaks.values() {
                    self.data.push(leak.clone());
                }
            }
            Err(_) => {}
        }
    }
}
