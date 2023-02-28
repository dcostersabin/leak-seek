use crate::get_stop_words;
use crate::shannon_entropy;
use crate::Fragment;
use crate::Leak;
use crate::Rule;
use fancy_regex::Regex;
use std::{process::Command, str::from_utf8};

#[derive(Debug)]
pub struct MatchPipeline {
    pub leaks: Vec<Leak>,
    git: bool,
    stop_words: Vec<String>,
}

impl MatchPipeline {
    pub fn new(git: bool) -> Self {
        Self {
            git: git,
            leaks: Vec::new(),
            stop_words: get_stop_words(),
        }
    }

    fn has_stop_words(&mut self, leak: &String) -> bool {
        let mut flag = false;
        self.stop_words.iter().for_each(|keyword| {
            if leak.to_lowercase().contains(keyword) {
                flag = true;
            }
        });
        return flag;
    }

    pub fn search(&mut self, rule: &Rule, fragment: &Fragment) {
        let re = Regex::new(&rule.regex);
        match re {
            Ok(re) => {
                let found_matches = re.find_iter(&fragment.body);
                for found_match in found_matches {
                    match found_match {
                        Ok(found_match) => {
                            let leak = &found_match.as_str().replace("\"", "").replace("\\", "");
                            let secret = self.get_secret(found_match.as_str());
                            let line_no = self.get_line_no(
                                leak,
                                &fragment.commit_id,
                                &fragment.parent_path,
                                self.git,
                            );

                            let entropy: f32 = shannon_entropy(secret.to_lowercase().as_str());
                            if (rule.id == "generic-api-key") && entropy < 3.6 {
                                continue;
                            }
                            if self.has_stop_words(&secret) {
                                continue;
                            }

                            let new_leak = Leak::new(
                                fragment.commit_id.as_str(),
                                fragment.file.as_str(),
                                line_no.as_str(),
                                leak.as_str().replace("\n", "").trim(),
                                secret.as_str(),
                                fragment.author.as_str(),
                                rule.id.as_str(),
                                entropy,
                            );
                            self.leaks.push(new_leak);
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
    }
    fn get_line_no_git(&mut self, secret: &str, path: &str) -> String {
        let subcommand = format!(
            "cat {} | grep -Ein \"{}\" | cut -f'1' -d':'",
            path,
            secret.trim().replace("''", "\'"),
        );
        let command_args = &["-c", subcommand.as_str()];

        let output = Command::new("/bin/bash").args(command_args).output();
        match &output {
            Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                Ok(line) => {
                    let line_nos: Vec<&str> = line.split("\n").collect();
                    let mut lines: Vec<&str> = Vec::new();
                    for line in line_nos {
                        if !line.trim().is_empty() {
                            lines.push(line.trim());
                        }
                    }
                    if lines.len() > 0 {
                        return lines.join(",");
                    }
                    return "N/A".to_string();
                }
                Err(_) => {
                    return "N/A".to_string();
                }
            },
            Err(_) => {
                return "N/A".to_string();
            }
        }
    }

    fn get_line_no(&mut self, secret: &str, commit_id: &str, path: &str, git: bool) -> String {
        if !git {
            return self.get_line_no_git(secret, path);
        }
        let subcommand = format!(
            "git --no-pager grep -Ein \"{}\" {} | cut -f'3' -d':'",
            secret.trim().replace("''", "\'"),
            commit_id
        );
        let command_args = &["-c", subcommand.as_str()];

        let output = Command::new("/bin/bash")
            .current_dir(&path)
            .args(command_args)
            .output();

        match &output {
            Ok(content) => match from_utf8(&content.stdout.to_ascii_lowercase()) {
                Ok(line) => {
                    let line_nos: Vec<&str> = line.split("\n").collect();
                    let mut lines: Vec<&str> = Vec::new();
                    for line in line_nos {
                        if !line.trim().is_empty() {
                            lines.push(line.trim());
                        }
                    }
                    if lines.len() > 0 {
                        return lines.join(",");
                    }
                    return "N/A".to_string();
                }
                Err(_) => {
                    return "N/A".to_string();
                }
            },
            Err(_) => {
                return "N/A".to_string();
            }
        }
    }

    fn get_secret(&mut self, leak: &str) -> String {
        if leak.contains(":") {
            let col: Vec<&str> = leak.split(":").collect();
            if col.len() > 1 {
                return col[1]
                    .replace("\n", "")
                    .replace("'", "")
                    .replace(">", "")
                    .replace(".", "")
                    .replace("\\", "")
                    .replace("\"", "")
                    .trim()
                    .to_string();
            } else {
                return col[0]
                    .replace("\n", "")
                    .replace("'", "")
                    .replace("\\", "")
                    .replace(">", "")
                    .replace("\"", "")
                    .replace(".", "")
                    .trim()
                    .to_string();
            }
        }

        if leak.contains("=") {
            let eq: Vec<&str> = leak.split("=").collect();

            if eq.len() > 1 {
                return eq[1]
                    .replace("\n", "")
                    .replace("'", "")
                    .replace(">", "")
                    .replace(".", "")
                    .replace("\"", "")
                    .replace("\\", "")
                    .trim()
                    .to_string();
            } else {
                return eq[0]
                    .replace("\n", "")
                    .replace("'", "")
                    .replace("\\", "")
                    .replace(">", "")
                    .replace("\"", "")
                    .replace(".", "")
                    .trim()
                    .to_string();
            }
        }

        return leak
            .replace("\n", "")
            .replace("'", "")
            .replace("\"", "")
            .replace("\"", "")
            .replace(">", "")
            .replace(".", "")
            .trim()
            .to_string();
    }
}
