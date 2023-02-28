use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leak {
    pub commit_id: String,
    pub file: String,
    pub line: String,
    pub leak_key: String,
    pub secret: String,
    pub author: String,
    pub match_type: String,
    pub entropy: f32,
}

impl Leak {
    pub fn new(
        commit_id: &str,
        file: &str,
        line: &str,
        leak_key: &str,
        secret: &str,
        author: &str,
        match_type: &str,
        entropy: f32,
    ) -> Self {
        Self {
            commit_id: commit_id.to_string(),
            file: file.to_string(),
            line: line.to_string(),
            leak_key: leak_key.to_string(),
            secret: secret.to_string(),
            author: author.to_string(),
            match_type: match_type.to_string(),
            entropy: entropy,
        }
    }
}
