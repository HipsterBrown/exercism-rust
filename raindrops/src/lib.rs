pub fn raindrops(num: u64) -> String {
    let mut output = String::new();

    if num % 3 == 0 {
        output.push_str("Pling");
    }

    if num % 5 == 0 {
        output.push_str("Plang");
    }

    if num % 7 == 0 {
        output.push_str("Plong");
    }

    if output.is_empty() {
        output.push_str(num.to_string().as_str());
    }
    output
}
