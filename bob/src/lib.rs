fn string_is_uppercase(input: &String) -> bool {
    let mut result = true;

    for c in input.chars() {
        match c {
            'a'...'z' => {
                result = false;
            }
            _ => { }
        }
    }

    result
}

pub fn reply(input: &str) -> &str {
    let input_string = String::from(input);

    if input_string.is_empty() {
        return "Fine. Be that way!";
    }

    if string_is_uppercase(&input_string) {
        return "Whoa, chill out!";
    }

    if input_string.ends_with("?") {
        return "Sure.";
    }

    "Whatever."
}
