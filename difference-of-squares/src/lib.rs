pub fn square_of_sum(base: u64) -> u64 {
    (1..base+1).fold(0, |accum, i| accum + i).pow(2)
}

pub fn sum_of_squares(base: u64) -> u64 {
    (1..base+1).map(|i| i.pow(2)).fold(0, |accum, i| accum + i)
}

pub fn difference(base: u64) -> u64 {
   square_of_sum(base) - sum_of_squares(base)
}
