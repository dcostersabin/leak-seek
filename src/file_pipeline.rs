use crate::get_fragment;
use crate::get_rules;
use crate::Leak;
use crate::MatchPipeline;
use crate::Rule;
use rayon::prelude::*;
use std::collections::hash_map::HashMap;
use std::sync::{Arc, Mutex};

use walkdir::WalkDir;
pub struct FilePipeline {
    path: String,
    rules: Vec<Rule>,
    pub data: Vec<Leak>,
}

impl FilePipeline {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            rules: get_rules(),
            data: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.run()
    }

    fn get_files(&mut self) -> Vec<String> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
            let filename = format!("{}", entry.path().display().to_string().as_str());
            files.push(filename);
        }
        return files;
    }

    fn run(&mut self) {
        self.search()
    }

    fn search(&mut self) {
        let files: Vec<String> = self.get_files();
        let shared_leaks: HashMap<String, Leak> = HashMap::new();
        let shared_data = Arc::new(Mutex::new(shared_leaks));
        files.par_iter().for_each(|file| {
            let fragment = get_fragment(file);
            self.rules.par_iter().for_each(|rule| {
                rule.keywords.par_iter().for_each(|keyword| {
                    if fragment.body.to_lowercase().contains(keyword) {
                        let mut match_obj = MatchPipeline::new(false);
                        match_obj.search(rule, &fragment);

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
