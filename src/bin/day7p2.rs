use std::io::{self, BufRead};

fn main() {
    let positions: Vec<i64> = io::stdin()
        .lock()
        .split(b',')
        .map(Result::unwrap)
        .map(|input| std::str::from_utf8(&input).unwrap().trim().parse().unwrap())
        .collect();

    // we want to minimize the sum of all fuel costs
    // which is a sum of parabolas concaved up (leaving target position free)
    // which is itself a parabola concaved up
    // sooooo we can walk around it to find the minimum

    let mut target_pos = mean(&positions); // not optimal, but close to it
    let mut target_fuel_usage = fuel_usage(&positions, target_pos);

    // try going down
    while target_pos > 1 && fuel_usage(&positions, target_pos - 1) < target_fuel_usage {
        target_pos -= 1;
        target_fuel_usage = fuel_usage(&positions, target_pos);
    }

    // try going up
    while target_pos < i64::MAX && fuel_usage(&positions, target_pos + 1) < target_fuel_usage {
        target_pos += 1;
        target_fuel_usage = fuel_usage(&positions, target_pos);
    }

    // could possibly be further optimized to use a variable step size

    dbg!(target_fuel_usage);
}

fn fuel_usage(positions: &[i64], target_pos: i64) -> i64 {
    positions
        .iter()
        .map(|&p| (target_pos - p).abs())
        // step n costs n fuel
        // all steps cost <sum numbers from 1 to n> fuel
        .map(|n_steps| n_steps * (n_steps + 1) / 2)
        .sum::<i64>()
}

fn mean(v: &[i64]) -> i64 {
    let sum = v.iter().sum::<i64>();
    let count = v.len() as i64;

    (sum + (count / 2)) / count
}
