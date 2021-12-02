use std::io::{self, BufRead};

fn main() {

    let nums: Vec<u64> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse().expect("int parse error"))
        .collect();

    #[derive(Default)]
    struct State {
        prev: Option<Vec<u64>>,
        count: usize,
    }

    let State {count, ..} = nums.windows(3)
        .fold(State::default(), |State {prev, mut count}, x| {
            if let Some(prev) = prev {
                if x.iter().sum::<u64>() > prev.iter().sum::<u64>() {
                    count += 1;
                }
            }

            State { prev: Some(x.to_owned()) , count }
        });

    dbg!(count);
}


