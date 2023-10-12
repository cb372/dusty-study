use std::collections::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;

mod data;

use data::generated::build_map;

static DB: Lazy<HashMap<&str, Vec<& 'static str>>> = Lazy::new(|| {
    build_map()
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
