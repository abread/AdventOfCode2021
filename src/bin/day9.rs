use std::io::{self, BufRead};

fn main() {
    let map: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // part 1
    dbg!(iter_enumerate_2d(&map)
        .filter(|&(coord, _v)| is_low_point(&map, coord))
        .map(|(_coord, &v)| v as u64 + 1) // calc risk level
        .sum::<u64>());

    // part 2
    let mut next_id: u32 = 0;
    let mut basin_map: Vec<Vec<_>> = map
        .iter()
        .map(|col| col.iter().map(|_| next_id).collect())
        .collect();

    // give every basin an ID
    for ((l, c), _v) in iter_enumerate_2d(&map).filter(|&(coord, _v)| is_low_point(&map, coord)) {
        basin_map[l][c] = next_id;
        next_id += 1;
    }
    // mark all high points as non-basins
    for ((l, c), _v) in iter_enumerate_2d(&map).filter(|&(_, &v)| v == 9) {
        basin_map[l][c] = u32::MAX;
    }

    let mut was_modified: bool = true;

    // let basins grow from low points
    while was_modified {
        was_modified = false;

        for ((l, c), _) in iter_enumerate_2d(&map) {
            let basin_id = basin_map[l][c];
            if basin_id == u32::MAX {
                continue;
            }

            for (l2, c2) in iter_adj_pos((l, c), &map) {
                if basin_map[l2][c2] < basin_id {
                    // grow basins deterministically
                    // even if there are overlaps, they will merge properly
                    basin_map[l2][c2] = basin_id;
                    was_modified = true;
                }
            }
        }
    }

    let mut basin_sizes = basin_map.iter().flat_map(|col| col.iter()).copied().fold(
        vec![0_u64; (next_id + 1) as usize],
        |mut counts, id| {
            if id <= next_id {
                counts[id as usize] += 1;
            }

            counts
        },
    );
    basin_sizes.sort_unstable();

    dbg!(basin_sizes.iter().rev().take(3).product::<u64>());
}

fn is_low_point(map: &[Vec<u8>], (l, c): (usize, usize)) -> bool {
    let point_value = map[l][c];
    iter_adj_pos((l, c), map)
        .map(|(l, c)| map[l][c])
        .all(|v| v > point_value)
}

fn iter_enumerate_2d<T>(map: &[Vec<T>]) -> impl Iterator<Item = ((usize, usize), &T)> {
    map.iter().enumerate().flat_map(|(l_idx, col)| {
        col.iter()
            .enumerate()
            .map(move |(c_idx, v)| ((l_idx, c_idx), v))
    })
}

fn iter_adj_pos<T>((l, c): (usize, usize), map: &[Vec<T>]) -> impl Iterator<Item = (usize, usize)> {
    let top = if l > 0 { Some((l - 1, c)) } else { None };

    let bottom = if l < map.len() - 1 {
        Some((l + 1, c))
    } else {
        None
    };

    let left = if c > 0 { Some((l, c - 1)) } else { None };

    let right = if c < map[0].len() - 1 {
        Some((l, c + 1))
    } else {
        None
    };

    maybe_once(top)
        .chain(maybe_once(bottom))
        .chain(maybe_once(left))
        .chain(maybe_once(right))
}

fn maybe_once<T>(v: Option<T>) -> impl Iterator<Item = T> {
    std::iter::once(v).flatten()
}
