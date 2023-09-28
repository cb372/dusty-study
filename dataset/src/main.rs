use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{Write, Error};
use itertools::Itertools;
use codegen::Scope;

fn read_words(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn to_anagram_key(word: String) -> String {
    word
        .to_uppercase()
        .replace(" ", "")
        .chars()
        .sorted()
        .collect::<String>()
}

fn build_map(words: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

    for word in &words {
        let key = to_anagram_key(word.clone());
        let entry = hashmap.entry(key).or_insert_with(|| vec![]);
        entry.push(word.clone());
    }

    // remove any entries with singleton list values
    // because it means those words have no anagrams
    //
    // -- stopped doing this, because we often the words
    // in the crossword clue are not a real phrase in our dataset,
    // e.g. "so I sync diary" -> idiosyncrasy
    //hashmap.retain(|_, v| v.len() > 1);

    hashmap
}

fn generate_code(hashmap: HashMap<String, Vec<String>>) -> Scope {
    let mut scope = Scope::new();

    scope.import("std::collections", "HashMap");

    let function = scope
        .new_fn("build_map")
        .allow("dead_code")
        .ret("HashMap<& 'static str, Vec<& 'static str>>");

    function.line("HashMap::from([");
    for (k, vs) in hashmap {
        let values = vs
            .into_iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<String>>()
            .join(", ");
        function.line(format!("(\"{}\", vec![{}])", k, values));
    }
    function.line("])");

    scope
}

fn main() -> Result<(), Error> {
    let mut words = read_words("./words-uniq.txt");
    println!("{} lines read from input file", words.len());

    // filter out any words > 15 chars (excluding spaces)
    // because they won't fit in a crossword
    words.retain(|x| x.replace(" ", "").chars().count() <= 15);
    println!("{} remaining after filtering out long words", words.len());

    // build hashmap of anagram keys to lists of matching words
    let hashmap = build_map(words);
    println!("{} anagram keys", hashmap.len());

    let scope = generate_code(hashmap);
    println!("Generated code");

    let mut output = File::create("../netlify/functions/hello/src/database/data/generated.rs")?;
    write!(output, "{}", scope.to_string())
}
