use std::{
    collections::BinaryHeap,
    io::{self, BufRead},
};

fn main() {
    let map = parse_input(io::stdin().lock().lines().map(Result::unwrap));
    dbg!("part1", search_min_path(&map));

    let aug_map = augment_map(&map);
    dbg!("part2", search_min_path(&aug_map));
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = lines
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .filter(|v: &Vec<u8>| !v.is_empty())
        .collect();

    for v in &mut res {
        v.shrink_to_fit();
    }
    res.shrink_to_fit();
    res
}

fn search_min_path(map: &[Vec<u8>]) -> isize {
    // Djikstra, see std::collections::binary_heap
    let mut dist: Vec<Vec<isize>> = map.iter().map(|l| vec![isize::MAX; l.len()]).collect();
    let mut prev: Vec<Vec<(usize, usize)>> = map
        .iter()
        .map(|l| vec![(usize::MAX, usize::MAX); l.len()])
        .collect();

    // "the starting position is never entered, so its risk is not counted"
    dist[0][0] = 0_isize;
    prev[0][0] = (0, 0);

    // stores (-dist[x][y], (x, y)) pairs
    let mut queue = BinaryHeap::new();
    queue.push((-dist[0][0], (0, 0)));

    while let Some((dist_node_inv, (x, y))) = queue.pop() {
        let dist_node = -dist_node_inv; // we invert costs in the heap to find the shortest path

        if (x, y) == (map.len() - 1, map[x].len() - 1) {
            // found it (the lowest cost path)! stop searching
            return dist_node;
        }

        if dist_node > dist[x][y] {
            // we already found a better path, don't process
            continue;
        }

        for (neigh_x, neigh_y) in iter_neighbors(x, y, map.len(), map[x].len()) {
            let d = dist_node + map[neigh_x][neigh_y] as isize;
            if d < dist[neigh_x][neigh_y] {
                dist[neigh_x][neigh_y] = d;
                prev[neigh_x][neigh_y] = (x, y);
                queue.push((-d, (neigh_x, neigh_y)));
            }
        }
    }

    unreachable!()
}

fn iter_neighbors(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    macro_rules! pos {
        ($i:ident - 1, $($j:tt)*) => {
            if $i > 0 { let i = $i-1; pos!(i, $($j)*) } else { None }
        };
        ($i:ident + 1, $($j:tt)*) => {{
            let i = $i + 1;
            if i < max_x { pos!(i, $($j)*) } else { None }
        }};
        ($i:ident, $(j:tt)*) => { pos!($i, $($j)*) };

        ($i:ident, $j:ident - 1) => {
            if $j > 0 { let j = $j-1; pos!($i, j) } else { None }
        };
        ($i:ident, $j:ident + 1) => {{
            let j = $j + 1;
            if j < max_y { pos!($i, j) } else { None }
        }};
        ($i:ident, $j:ident) => {
            Some(($i, $j))
        };
    }

    use std::iter::once;
    once(pos!(x - 1, y))
        .chain(once(pos!(x, y - 1)))
        .chain(once(pos!(x, y + 1)))
        .chain(once(pos!(x + 1, y)))
        .flatten() // remove Nones
}

fn augment_map(map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut aug_map = Vec::with_capacity(map.len() * 5);
    for _ in 0..5 {
        for v in map {
            aug_map.push(vec![u8::MAX; v.len() * 5]);
        }
    }

    for x_off in 0..5 {
        for y_off in 0..5 {
            for (x, y, val) in map
                .iter()
                .enumerate()
                .flat_map(|(x, v)| v.iter().copied().enumerate().map(move |(y, v)| (x, y, v)))
            {
                aug_map[x + x_off * map.len()][y + y_off * map[x].len()] =
                    (val - 1 + x_off as u8 + y_off as u8) % 9 + 1;
            }
        }
    }

    assert!(aug_map.iter().all(|line| line.iter().all(|&v| v < 10)));

    aug_map
}
