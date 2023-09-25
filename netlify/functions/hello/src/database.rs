use std::collections::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;

static DB: Lazy<HashMap<&str, Vec<String>>> = Lazy::new(|| {
    HashMap::from([
        ("ACT", vec!["ACT".to_string(), "CAT".to_string()]),
        ("DSTUY", vec!["DUSTY".to_string(), "STUDY".to_string()])
    ])
});

pub fn find(input: String) -> Vec<String> {
    let key = input
        .to_uppercase()
        .replace(" ", "")
        .chars()
        .sorted()
        .collect::<String>();

    DB
        .get(key.as_str())
        .map(|v| v.to_vec())
        .unwrap_or_else(|| vec![])
}
