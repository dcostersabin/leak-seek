use regex::Regex;
pub fn get_file_name(content: &str) -> String {
    let mut filename = "".to_string();
    let re = Regex::new("diff --git .*.\\n");
    match re {
        Ok(result) => {
            let re_matches = result.find(content);
            match re_matches {
                Some(line) => {
                    let line_split: Vec<&str> = line.as_str().split(" ").collect();
                    let last_element = line_split.len() - 1;
                    filename = line_split[last_element]
                        .replace("a/", "")
                        .replace("b/", "")
                        .replace("\n", "");
                    return filename;
                }
                None => {
                    return filename;
                }
            }
        }
        Err(_) => {
            return filename;
        }
    }
}
