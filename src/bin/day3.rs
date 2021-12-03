use std::io::{self, BufRead};

fn main() {
    let mut counts: Vec<isize> = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        // ensure counts is big enough
        counts.resize(line.len(), 0);

        for (idx, bit) in line.chars().enumerate() {
            let diff = match bit {
                '1' => 1_isize,
                '0' => -1_isize,
                _ => unreachable!(),
            };
            counts[idx] += diff;
        }
    }

    dbg!(&counts);

    let gamma_bits = |&count| if count > 0 { 1u8 } else { 0 };
    let epsilon_bits = |&count| if count < 0 { 1u8 } else { 0 };
    let bit_folder = |acc, bit| (acc << 1) + bit as u64;

    let gamma = counts.iter().map(gamma_bits).fold(0u64, bit_folder);

    let epsilon = counts.iter().map(epsilon_bits).fold(0u64, bit_folder);

    dbg!(gamma * epsilon);
}
