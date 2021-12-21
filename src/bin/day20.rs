use std::{
    collections::HashSet,
    io::{self, BufRead},
    ops::Range,
};

use itertools::Itertools;

fn main() {
    let (mut image, algo_map) = parse_input(io::stdin().lock().lines().map(Result::unwrap));
    print_image(&image, (-15..50, -15..50));
    println!();

    for _ in 0..50 {
        image = enhance(image, &algo_map);

        print_image(&image, (-15..50, -15..50));
        println!();
    }

    assert!(!image.1); // can't be inverted in the end
    dbg!(image.0.len());
}

fn parse_input(
    mut lines: impl Iterator<Item = String>,
) -> ((HashSet<(isize, isize)>, bool), Vec<bool>) {
    let mut algo_map = Vec::with_capacity(512);
    for line in &mut lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        line.chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => unreachable!("parse error"),
            })
            .for_each(|b| algo_map.push(b));
    }

    let image = lines
        .enumerate()
        .flat_map(|(i, line)| {
            line.into_bytes()
                .into_iter()
                .filter(|&b| b != b'\n')
                .enumerate()
                .filter(|(_j, b)| *b == b'#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect();

    ((image, false), algo_map)
}

fn enhance(
    (image, inverted): (HashSet<(isize, isize)>, bool),
    algo_map: &[bool],
) -> (HashSet<(isize, isize)>, bool) {
    let mut new = HashSet::new();
    let new_inverted = if inverted {
        algo_map[0b111_111_111]
    } else {
        algo_map[0]
    };

    for (x, y) in image
        .iter()
        .flat_map(|&(x, y)| (x - 1..=x + 1).cartesian_product(y - 1..=y + 1))
    {
        let n = (x - 1..=x + 1)
            .cartesian_product(y - 1..=y + 1)
            .map(|(x, y)| {
                if image.contains(&(x, y)) != inverted {
                    1_usize
                } else {
                    0
                }
            })
            .fold(0_usize, |acc, x| acc * 2 + x);

        if algo_map[n] != new_inverted {
            new.insert((x, y));
        }
    }

    (new, new_inverted)
}

fn print_image(
    (image, inverted): &(HashSet<(isize, isize)>, bool),
    viewport: (Range<isize>, Range<isize>),
) {
    for x in viewport.0 {
        for y in viewport.1.clone() {
            if image.contains(&(x, y)) != *inverted {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
