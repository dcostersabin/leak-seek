#[derive(Debug)]
pub struct CommitInfo {
    pub commit_id: String,
    pub author: String,
    pub commit_date: String,
    content: String,
    pub body: String,
}

impl CommitInfo {
    pub fn new(content: &str) -> Self {
        Self {
            commit_id: "N/A".to_string(),
            author: "N/A".to_string(),
            commit_date: "N/A".to_string(),
            content: content.to_string(),
            body: "".to_string(),
        }
    }

    pub fn get_info(&mut self) {
        let lines: Vec<&str> = self.content.split("\n").collect();
        if lines.len() < 2 {
            return;
        }
        self.commit_id = lines[0].replace("commit", "").trim().to_string();
        self.author = lines[1].replace("Author:", "").trim().to_string();
        self.commit_date = lines[2].replace("Date:", "").trim().to_string();
        self.body = lines[3..lines.len() - 1].join("\n");
    }

}
