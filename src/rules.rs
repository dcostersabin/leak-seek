use serde::Deserialize;
extern crate toml;

#[derive(Deserialize, Debug)]
pub struct GitLeaksRules {
    stoplist: Stoplist,
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub description: String,
    pub id: String,
    pub regex: String,
    pub keywords: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct Stoplist {
    pub stopwords: Vec<String>,
}

pub fn get_rules() -> Vec<Rule> {
    let rule_string = include_str!("../rules/rules.toml");

    let gitleaks: GitLeaksRules = toml::from_str(rule_string).unwrap();

    return gitleaks.rules;
}

pub fn get_stop_words() -> Vec<String> {
    let rule_string = include_str!("../rules/rules.toml");

    let gitleaks: GitLeaksRules = toml::from_str(rule_string).unwrap();

    return gitleaks.stoplist.stopwords;
}
