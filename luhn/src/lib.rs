use std::iter::FromIterator;
pub fn is_valid(num_str: &str) -> bool {
    if num_str.trim().len() <= 1 || !valid_format(num_str) { 
        return false;
    }
    
    num_str
        .chars()
        .rev()
        .filter(|c| c.is_numeric())
        .enumerate()
        .map(|(index, num_char)| {
            let mut c: u32 = String::from_iter(vec![num_char]).parse().unwrap();
            if (index+1) % 2 == 0 {
                c *= 2;
                if c > 9 { c -= 9; }
            }
            c
        })
        .fold(0u32, |acc, n| acc.wrapping_add(n)) % 10 == 0
}

fn valid_format(num_str: &str) -> bool {
    num_str
        .split(" ")
        .flat_map(|s| s.chars())
        .all(|c| c.is_numeric())
}
