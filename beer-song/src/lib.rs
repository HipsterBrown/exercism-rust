fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

fn get_bottle_s(num: &u32) -> String {
    let mut output = String::from("bottle");
    if *num != 1 { output += "s" }

    let start = if *num == 0 { String::from("no more") } else { num.to_string() };
    format!("{} {}", start, output)
}

pub fn verse(line: u32) -> String {
    if line == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    let new_line = line - 1;
    let it_or_one = if line == 1 { "it" } else { "one" };
    format!(
        "{0} of beer on the wall, {1} of beer.\nTake {3} down and pass it around, {2} of beer on the wall.\n",
        capitalize(get_bottle_s(&line)),
        get_bottle_s(&line),
        get_bottle_s(&new_line),
        it_or_one,
    )
}

pub fn sing(start_line: u32, end_line: u32) -> String {
    let mut output = String::new();
    for n in (end_line..(start_line + 1)).rev() {
        if n != start_line { output += "\n" }
        output += verse(n).as_str();
    }
    output
}
