use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let initial: Vec<char> = lines.next().unwrap().trim().chars().collect();
    lines.next().unwrap(); // empty line
    let rules = lines
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (before, after) = line.split_once(" -> ").unwrap();

            let c1 = before.chars().next().unwrap();
            let c2 = before.chars().nth(1).unwrap();

            ((c1, c2), after.chars().next().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let frequencies = polymer_freqs_after_steps(&initial, &rules, 10);
    let (_, &min) = frequencies.iter().min_by_key(|(_, &count)| count).unwrap();
    let (_, &max) = frequencies.iter().max_by_key(|(_, &count)| count).unwrap();

    dbg!("part1", max - min);

    // we can just compute everything again, it's that cheap

    let frequencies = polymer_freqs_after_steps(&initial, &rules, 40);
    let (_, &min) = frequencies.iter().min_by_key(|(_, &count)| count).unwrap();
    let (_, &max) = frequencies.iter().max_by_key(|(_, &count)| count).unwrap();

    dbg!("part2", max - min);
}

fn polymer_freqs_after_steps(
    polymer: &[char],
    rules: &HashMap<(char, char), char>,
    n: usize,
) -> HashMap<char, usize> {
    let mut pair_counts = rules
        .keys()
        .copied()
        .map(|(a, b)| ((a, b), 0_usize))
        .collect::<HashMap<_, _>>();

    for win in polymer.windows(2) {
        let start = win[0];
        let end = win[1];
        *pair_counts.get_mut(&(start, end)).unwrap() += 1;
    }

    for _ in 0..n {
        let mut next_pair_counts = pair_counts.clone();
        for (&(start, end), &count) in &pair_counts {
            *next_pair_counts.get_mut(&(start, end)).unwrap() -= count;

            let middle = rules[&(start, end)];
            *next_pair_counts.get_mut(&(start, middle)).unwrap() += count;
            *next_pair_counts.get_mut(&(middle, end)).unwrap() += count;
        }

        pair_counts = next_pair_counts;
    }

    let mut counts = HashMap::new();
    for ((start, _end), count) in pair_counts {
        *counts.entry(start).or_default() += count;
    }

    // add last letter too!
    *counts.entry(*polymer.last().unwrap()).or_default() += 1;

    counts
}
