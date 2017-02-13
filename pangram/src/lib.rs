pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .split(|c| {
            match c {
                'a'...'z' => false,
                _ => true,
            }
        })
        .flat_map(|word| word.chars())
        .fold(Vec::new(), |mut accum, c| {
            if !accum.contains(&c) {
                accum.push(c);
            }
            accum
        })
        .len() == 26
}
