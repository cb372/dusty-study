use std::collections::HashMap;

#[allow(dead_code)]
pub fn build_map() -> HashMap<& 'static str, Vec<& 'static str>> {
    HashMap::from([
        ("ACT", vec!["ACT", "CAT"]),
        ("DSTUY", vec!["DUSTY", "STUDY"])
    ])
}
