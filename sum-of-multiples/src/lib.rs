pub fn sum_of_multiples(base: u32, multi: &Vec<u32>) -> u32 {
    let mut list: Vec<u32> = Vec::new();
    for number in 1..base {
        if let Some(_) = multi.iter().find(|&&x| number % x == 0) {
            list.push(number);
        }
    }
    list.iter().sum()
}
