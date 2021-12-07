use std::io::{self, BufRead};

fn main() {
    let mut positions: Vec<i64> = io::stdin()
        .lock()
        .split(b',')
        .map(Result::unwrap)
        .map(|input| std::str::from_utf8(&input).unwrap().trim().parse().unwrap())
        .collect();

    // compute median
    positions.sort_unstable();
    let optimal = positions[positions.len().div_euclid(2)];

    dbg!(positions.iter().map(|&p| (optimal - p).abs()).sum::<i64>());
}
