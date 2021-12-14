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
            dbg!(&line);
            let (before, after) = line.split_once(" -> ").unwrap();

            let c1 = before.chars().next().unwrap();
            let c2 = before.chars().nth(1).unwrap();

            ((c1, c2), after.chars().next().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let mut polymer = initial;
    for _ in 0..10 {
        polymer = step_polymer(polymer, &rules);
    }

    let frequencies = el_frequencies(&polymer);
    let (_, &min) = frequencies.iter().min_by_key(|(_, &count)| count).unwrap();
    let (_, &max) = frequencies.iter().max_by_key(|(_, &count)| count).unwrap();

    dbg!("part1", max - min);

    for _ in 10..40 {
        polymer = step_polymer(polymer, &rules);
    }
    let frequencies = el_frequencies(&polymer);
    let (_, &min) = frequencies.iter().min_by_key(|(_, &count)| count).unwrap();
    let (_, &max) = frequencies.iter().max_by_key(|(_, &count)| count).unwrap();

    dbg!("part2", max - min);
}

fn step_polymer(polymer: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    polymer
        .windows(2)
        .flat_map(|v| match *v {
            [a, b, ..] => [a, rules[&(a, b)]],
            _ => unreachable!(),
        })
        .chain(std::iter::once(polymer.iter().last().copied().unwrap()))
        .collect()
}

fn el_frequencies(polymer: &[char]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for &c in polymer {
        *map.entry(c).or_default() += 1;
    }

    map
}
