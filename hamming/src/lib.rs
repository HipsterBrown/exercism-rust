pub fn hamming_distance(first: &str, last: &str) -> Result<u8, &'static str> {
    if first.len() != last.len() { 
        return Err("Strings do not match length"); 
    }

    let mut count: u8 = 0;
    let mut last_chars = last.chars();
    for c in first.chars() {
        if Some(c) != last_chars.next() {
            count += 1;
        }
    }
    Ok(count)
}
