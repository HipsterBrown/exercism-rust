pub fn lsp(input: &str, span: usize) -> Result<u32, &str> {
    if input.chars().count() < span || !valid_format(input) {
        return Err("Nope");
    }

    if span == 0 {
        return Ok(1u32);
    }

    Ok(input.chars().collect::<Vec<char>>().windows(span).map(|window| {
        window.iter().map(|c| c.to_digit(10).unwrap()).product()
    })
    .max().unwrap())
}

fn valid_format(num_str: &str) -> bool {
    num_str
        .split(" ")
        .flat_map(|s| s.chars())
        .all(|c| c.is_numeric())
}
