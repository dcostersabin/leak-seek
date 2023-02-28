use crate::get_added;
use crate::get_diff_blocks;
use crate::get_file_name;
use crate::CommitInfo;
use crate::Fragment;
use regex::Regex;
use regex_split::RegexSplit;
use std::{process::Command, str::from_utf8};

pub struct Logs {
    pub path: String,
    pub logs: Vec<Fragment>,
}

impl Logs {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            logs: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.run()
    }

    fn run(&mut self) {
        let content = self.get_raw_logs();
        self.split_commits(content);
    }

    fn get_raw_logs(&mut self) -> String {
        let default = "".to_string();
        let command_args = &["log", "-p", "-U0", "--full-history", "--all"];

        let output = Command::new("git")
            .current_dir(&self.path)
            .args(command_args)
            .output();

        match &output {
            Ok(result) => match from_utf8(&result.stdout) {
                Ok(content) => {
                    return content.to_string();
                }
                Err(_) => return default,
            },
            Err(_) => return default,
        }
    }

    fn split_commits(&mut self, content: String) -> Vec<String> {
        let commits: Vec<String> = Vec::new();

        let re = Regex::new("commit \\b([a-f0-9]{40})\\b");
        match re {
            Ok(re) => {
                let commit_splits: Vec<&str> = re.split_inclusive_left(&content).collect();
                for commit in commit_splits {
                    if commit.trim().len() < 1 {
                        continue;
                    }
                    let mut info_obj = CommitInfo::new(&commit);
                    info_obj.get_info();
                    let diff_blocks = get_diff_blocks(&info_obj.body);
                    for diff in diff_blocks {
                        let filename = get_file_name(&diff);
                        if filename == "" {
                            continue;
                        };
                        let added_lines = get_added(&diff);
                        self.logs.push(Fragment::new(
                            &self.path,
                            &info_obj.commit_id,
                            &info_obj.author,
                            &info_obj.commit_date,
                            &filename,
                            &added_lines,
                        ));
                    }
                }
            }
            Err(_) => {}
        }

        return commits;
    }
}
