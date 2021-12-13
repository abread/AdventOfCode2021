use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn main() {
    let (mut matrix, fold_instructions) = parse_input();

    fold_along(&mut matrix, fold_instructions[0]);
    dbg!("part 1", matrix.len());

    for &instr in &fold_instructions[1..] {
        fold_along(&mut matrix, instr);
    }

    print_matrix(&matrix);
}

fn fold_along(matrix: &mut BTreeSet<(usize, usize)>, instr: FoldAlongInstr) {
    let to_fold: Vec<_> = matrix
        .iter()
        .copied()
        .filter(|&(x, y)| match instr {
            FoldAlongInstr::X(fold_x) => x > fold_x,
            FoldAlongInstr::Y(fold_y) => y > fold_y,
        })
        .collect();

    for (x, y) in to_fold {
        matrix.remove(&(x, y));

        match instr {
            FoldAlongInstr::X(fold_x) => matrix.insert((2 * fold_x - x, y)),
            FoldAlongInstr::Y(fold_y) => matrix.insert((x, 2 * fold_y - y)),
        };
    }
}

#[derive(Clone, Copy)]
enum FoldAlongInstr {
    X(usize),
    Y(usize),
}

fn parse_input() -> (BTreeSet<(usize, usize)>, Vec<FoldAlongInstr>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut line = lines.next().unwrap().unwrap();
    let mut grid = BTreeSet::new();
    while !line.trim().is_empty() {
        let (x, y) = line.trim().split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        grid.insert((x, y));
        line = lines.next().unwrap().unwrap();
    }

    let mut folds = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let (prefix, val) = line.trim().split_once('=').unwrap();
        let val = val.parse().unwrap();

        let val = match prefix {
            "fold along x" => FoldAlongInstr::X(val),
            "fold along y" => FoldAlongInstr::Y(val),
            _ => unreachable!(),
        };
        folds.push(val);
    }

    (grid, folds)
}

fn print_matrix(matrix: &BTreeSet<(usize, usize)>) {
    let max_x = matrix.iter().map(|&(x, _y)| x).max().unwrap();
    let max_y = matrix.iter().map(|&(_x, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if matrix.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
