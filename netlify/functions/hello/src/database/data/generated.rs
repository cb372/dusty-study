use std::collections::HashMap;

static JSON: &str = include_str!("anagrams.json");

#[allow(dead_code)]
pub fn build_map() -> HashMap<& 'static str, Vec<& 'static str>> {
    serde_json::from_str(JSON).unwrap()
}
