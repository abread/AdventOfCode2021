use std::io::{self, BufRead};

const N_ITER: usize = 256;
const N_BABIES_PER_FISH: usize = 1;

// Idea, don't track each lanternfish, track how many are in each clock cycle
fn main() {
    let mut fish_count_per_state = read_input();

    for _ in 0..N_ITER {
        let mut new_fish_count_per_state = [0_usize; 9];

        // decrement timer
        for state in (1..=8).rev() {
            new_fish_count_per_state[state - 1] = fish_count_per_state[state];
        }

        // reset timer for the pregnant fish
        new_fish_count_per_state[6] += fish_count_per_state[0];

        // spawn newborns (1 per pregnant fish)
        new_fish_count_per_state[8] += N_BABIES_PER_FISH * fish_count_per_state[0];

        fish_count_per_state = new_fish_count_per_state;
    }

    dbg!(fish_count_per_state.iter().sum::<usize>());
}

fn read_input() -> [usize; 9] {
    let mut fish_count_per_state = [0_usize; 9];

    for timer in io::stdin().lock().split(b',') {
        let timer = timer.unwrap();
        let timer = std::str::from_utf8(&timer).unwrap();
        let timer = timer.trim();
        let timer: usize = timer.parse().unwrap();

        fish_count_per_state[timer] += 1;
    }

    fish_count_per_state
}
