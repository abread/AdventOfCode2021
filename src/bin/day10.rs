use std::io::{self, BufRead};

fn main() {
    let input: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();

    // part 1
    dbg!(input
        .iter()
        .filter_map(|chunk| find_incorrect_closing(chunk))
        .map(wrong_char_score)
        .sum::<u64>());

    // part 2
    let mut scores: Vec<_> = input
        .iter()
        .filter(|chunk| find_incorrect_closing(chunk).is_none())
        .map(|chunk| compute_completion_seq(chunk))
        .map(completion_seq_score)
        .collect();
    scores.sort_unstable();

    dbg!(scores[scores.len() / 2]);
}

fn find_incorrect_closing(chunk: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in chunk.chars() {
        match c {
            '{' | '[' | '(' | '<' => stack.push(c),
            '}' => {
                if stack.pop() != Some('{') {
                    return Some(c);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Some(c);
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return Some(c);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Some(c);
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

fn compute_completion_seq(chunk: &str) -> String {
    let mut stack = Vec::new();
    for c in chunk.chars() {
        match c {
            '}' | ']' | ')' | '>' => assert_eq!(stack.pop(), Some(c)),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '<' => stack.push('>'),
            _ => unreachable!(),
        }
    }

    stack.into_iter().rev().collect()
}

fn wrong_char_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn completion_seq_score(seq: String) -> u64 {
    seq.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, x| acc * 5 + x)
}
