use std::collections::HashMap;

pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            word.chars().filter(|c| c.is_alphanumeric()).collect::<String>()
        })
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        })
}
