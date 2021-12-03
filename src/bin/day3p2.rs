use std::io::{self, BufRead};

fn main() {
    let input: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '1' => 1,
                    '0' => 0,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let o2_gen_rating_bit_criteria = |count| match count {
        1.. => 1,
        0 => 1,
        _ => 0,
    };
    let co2_scrub_rating_bit_criteria = |count| match count {
        1.. => 0,
        0 => 0,
        _ => 1,
    };

    let o2_gen_rating = compute_rating(o2_gen_rating_bit_criteria, input.clone());
    let co2_scrub_rating = compute_rating(co2_scrub_rating_bit_criteria, input);

    dbg!(dbg!(o2_gen_rating) * dbg!(co2_scrub_rating));
}

fn compute_rating(bit_criteria: impl Fn(isize) -> u8, mut input: Vec<Vec<u8>>) -> u64 {
    for idx in 0..input[0].len() {
        if input.len() <= 1 {
            break;
        }

        // compute count
        let count: isize = input
            .iter()
            .map(|bitstring| bitstring[idx])
            .map(|bit| match bit {
                1 => 1,
                0 => -1,
                _ => unreachable!(),
            })
            .sum();

        // filter
        let bit_to_keep = bit_criteria(count);
        input.retain(|bitstring| bitstring[idx] == bit_to_keep);
    }

    assert!(input.len() == 1);

    input[0]
        .iter()
        .fold(0u64, |acc, &bit| (acc << 1) + bit as u64)
}
