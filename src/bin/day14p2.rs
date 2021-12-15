use std::{
    collections::HashMap,
    io::{self, BufRead},
    sync::mpsc::channel,
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
    let rules = Box::leak(Box::new(rules)); // make a static ref

    let frequencies = polymer_freqs_after_steps(&initial, rules,40);
    let (_, &min) = frequencies.iter().min_by_key(|(_, &count)| count).unwrap();
    let (_, &max) = frequencies.iter().max_by_key(|(_, &count)| count).unwrap();

    dbg!("part2", max - min);
}

fn polymer_freqs_after_steps(polymer: &[char], rules: &'static HashMap<(char, char), char>, n: usize) -> HashMap<char, usize> {
    fn recurse(start: char, end: char, rules: &HashMap<(char, char), char>, n: usize, counts: &mut HashMap<char, usize>) {
        if n == 0 {
            *counts.entry(start).or_default() += 1;
        } else {
            let middle = rules[&(start, end)];
            recurse(start, middle, rules, n - 1, counts);
            recurse(middle, end, rules, n - 1, counts);
        }
    }

    let (tx, rx) = channel();
    for win in polymer.windows(2) {
        let start = win[0];
        let end = win[1];
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut counts_local = HashMap::new();
            recurse(start, end, rules, n, &mut counts_local);
            tx.send(counts_local).unwrap();
        });
    }
    std::mem::drop(tx); // make sure the channel is closed after the last thread completes

    let mut counts = HashMap::new();
    for count in rx.into_iter() {
        for (c, count) in count {
            *counts.entry(c).or_default() += count;
        }
    }

    // add last letter too!
    *counts.entry(*polymer.last().unwrap()).or_default() += 1;

    counts
}
