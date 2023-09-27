use std::collections::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;

static DB: Lazy<HashMap<&str, Vec<& 'static str>>> = Lazy::new(|| {
    HashMap::from([
        ("ACT", vec!["ACT", "CAT"]),
        ("DSTUY", vec!["DUSTY", "STUDY"])
    ])
});

pub fn find(input: &str) -> Vec<& 'static str> {
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
