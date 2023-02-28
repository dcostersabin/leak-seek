use crate::Fragment;
use std::fs::read_to_string;

pub fn get_fragment(path: &str) -> Fragment {
    let content = read_to_string(path);
    match content {
        Ok(content) => {
            return Fragment::new(path, "N/A", "N/A", "N/A", path, &content);
        }
        Err(_) => return Fragment::new(path, "N/A", "N/A", "N/A", path, ""),
    }
}
