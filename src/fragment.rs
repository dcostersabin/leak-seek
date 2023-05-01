#[derive(Debug)]
pub struct Fragment {
    pub parent_path: String,
    pub commit_id: String,
    pub author: String,
    pub date: String,
    pub file: String,
    pub body: String,
}

impl Fragment {
    pub fn new(
        parent_path: &str,
        commit_id: &str,
        author: &str,
        date: &str,
        file: &str,
        body: &str,
    ) -> Self {
        Self {
            parent_path: parent_path.to_string(),
            commit_id: commit_id.to_string(),
            author: author.to_string(),
            date: date.to_string(),
            file: file.to_string(),
            body: body.to_string(),
        }
    }
}
