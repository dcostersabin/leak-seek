use rayon::prelude::*;
use regex::Regex;
use regex_split::RegexSplit;
use std::sync::{Arc, Mutex};

pub fn get_diff_blocks(content: &str) -> Vec<String> {
    let diffs: Vec<String> = Vec::new();
    let shared_vec: Vec<String> = Vec::new();
    let shared_diffs = Arc::new(Mutex::new(shared_vec));

    let re = Regex::new("diff --git .*.\\n");

    match re {
        Ok(re) => {
            let diff_blocks: Vec<&str> = re.split_inclusive_left(content).collect();
            diff_blocks.par_iter().for_each(|diff| {
                if re.is_match(diff) {
                    match shared_diffs.lock() {
                        Ok(mut shared_diffs) => {
                            shared_diffs.push(diff.to_string());
                        }
                        Err(_) => {}
                    }
                }
            });
            match shared_diffs.lock() {
                Ok(shared_diffs) => {
                    return shared_diffs.to_vec();
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    return diffs;
}
