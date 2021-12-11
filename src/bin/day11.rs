use std::io::{self, BufRead};

fn main() {
    let mut matrix = parse_input(io::stdin().lock().lines().map(Result::unwrap));

    // part 1
    {
        let mut m1 = matrix.clone();
        let mut flash_count = 0_usize;
        for _ in 0..100 {
            flash_count += step(&mut m1);
        }

        dbg!("part 1 - flashes after 100 steps: ", flash_count);
    }

    // part 2
    for i in 1.. {
        step(&mut matrix);

        if matrix
            .iter()
            .flat_map(|r| r.iter())
            .all(|&level| level == 0)
        {
            dbg!("part 2 - all flash after (steps): ", i);
            break;
        }
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    lines
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .filter(|v: &Vec<u8>| !v.is_empty())
        .collect()
}

fn step(matrix: &mut [Vec<u8>]) -> usize {
    increase_energy_levels_for_all_by_one(matrix);

    // Redo flash updates until no more octopuses are flashing
    let mut flash_count = 0_usize;
    let mut partial_flash_count = 1_usize;
    while partial_flash_count > 0 {
        partial_flash_count = flash_update(matrix);
        flash_count += partial_flash_count;
    }

    reset_flashed_octopuses(matrix);
    flash_count
}

fn increase_energy_levels_for_all_by_one(matrix: &mut [Vec<u8>]) {
    for row in matrix {
        for energy_level in row {
            *energy_level += 1;
        }
    }
}

fn flash_update(matrix: &mut [Vec<u8>]) -> usize {
    let mut flash_count = 0_usize;

    // octopuses marked with 9 are about to flash, marked with 11 means they already flashed in this round
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 10 {
                //println!("flash {:?}", (i, j));
                matrix[i][j] = 11;
                flash_count += 1;

                // increment adjacent, without marking them as flashed
                for (adj_i, adj_j) in adj_pos((i, j), (matrix.len(), matrix[i].len())) {
                    //println!("updating pos {:?} adjacent to {:?}", (adj_i, adj_j), (i, j));
                    if matrix[adj_i][adj_j] < 10 {
                        matrix[adj_i][adj_j] += 1;
                    }
                }
            }
        }
    }

    flash_count
}

fn reset_flashed_octopuses(matrix: &mut [Vec<u8>]) {
    for row in matrix {
        for energy_level in row {
            if *energy_level == 11 {
                *energy_level = 0;
            }
        }
    }
}

fn adj_pos(
    (i, j): (usize, usize),
    (max_i, max_j): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    macro_rules! pos {
        ($i:ident - 1, $($j:tt)*) => {
            if $i > 0 { let i = $i-1; pos!(i, $($j)*) } else { None }
        };
        ($i:ident + 1, $($j:tt)*) => {{
            let i = $i + 1;
            if i < max_i { pos!(i, $($j)*) } else { None }
        }};
        ($i:ident, $(j:tt)*) => { pos!($i, $($j)*) };

        ($i:ident, $j:ident - 1) => {
            if $j > 0 { let j = $j-1; pos!($i, j) } else { None }
        };
        ($i:ident, $j:ident + 1) => {{
            let j = $j + 1;
            if j < max_j { pos!($i, j) } else { None }
        }};
        ($i:ident, $j:ident) => {
            Some(($i, $j))
        };
    }

    use std::iter::once;
    once(pos!(i - 1, j - 1))
        .chain(once(pos!(i - 1, j)))
        .chain(once(pos!(i - 1, j + 1)))
        .chain(once(pos!(i, j - 1)))
        .chain(once(pos!(i, j + 1)))
        .chain(once(pos!(i + 1, j - 1)))
        .chain(once(pos!(i + 1, j)))
        .chain(once(pos!(i + 1, j + 1)))
        .flatten() // remove Nones
}

#[allow(dead_code)]
fn print_matrix(m: &[Vec<u8>]) {
    for row in m {
        for v in row {
            print!("{:2} ", v);
        }
        println!();
    }
    println!();
}
