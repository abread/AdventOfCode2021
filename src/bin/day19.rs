use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use itertools::Itertools;

type Coord = [i32; 3];

fn main() {
    let mut scanners = parse_input(io::stdin().lock().lines().map(Result::unwrap));
    let scanner0 = scanners.remove(0);

    let mut sc = scanners.clone();

    let mut ref_beacons = scanner0.beacons.clone();
    let mut ref_beacon_dists = scanner0.beacon_distances();

    let mut pending = scanners;
    let mut done = Vec::new();

    while !pending.is_empty() {
        let mut p = Vec::with_capacity(pending.len());

        // constantly draining and replacing pending but oh well
        for mut s in pending.into_iter() {
            if s.fit_pos_rotation(&mut ref_beacons, &mut ref_beacon_dists).is_some() {
                dbg!(s.id, s.pos);
                done.push(s);
            } else {
                //dbg!("failed to process", s.id);
                p.push(s);
            }
        }
        pending = p;
    }

    dbg!(ref_beacons.len());

    for s in &mut sc {
        s.fit_pos_rotation(&mut ref_beacons, &mut ref_beacon_dists).unwrap();
    }

}

#[derive(Clone)]
struct Scanner {
    id: usize,
    pos: Coord,
    rotation: [[i32; 3]; 3],
    beacons: HashSet<Coord>,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            id: 0,
            pos: [0, 0, 0],
            rotation: [[1, 0, 0], [0, 1, 0], [0, 0, 1]], // identity matrix
            beacons: HashSet::new(),
        }
    }
}

type BeaconDists = HashMap<i32, HashSet<(Coord, Coord)>>;
impl Scanner {
    fn beacon_distances(&self) -> BeaconDists {
        let mut distances: BeaconDists =
            HashMap::with_capacity(self.beacons.len() * (self.beacons.len() - 1));

        calc_beacon_dists(&self.beacons, &mut distances);
        distances
    }

    fn fit_pos_rotation(
        &mut self,
        ref_beacons: &mut HashSet<Coord>,
        ref_beacon_dists: &mut BeaconDists,
    ) -> Option<()> {
        match self.find_pos_rotation(ref_beacons, ref_beacon_dists) {
            Some((scanner_pos, rot)) => {
                self.pos = scanner_pos;
                self.rotation = rot;

                // grow reference set
                let scanner_pos_rot = apply_transform(&self.rotation, &self.pos);
                let inv_rot = invert_3x3(&self.rotation);

                for b in self
                    .beacons
                    .iter()
                    .map(|pos| apply_transform(&inv_rot, &vector_add(pos, &scanner_pos_rot)))
                {
                    ref_beacons.insert(b);
                }

                calc_beacon_dists(ref_beacons, ref_beacon_dists);
                Some(())
            }
            None => None,
        }
    }

    fn find_pos_rotation(
        &self,
        ref_beacons: &HashSet<Coord>,
        ref_beacon_dists: &BeaconDists,
    ) -> Option<(Coord, [[i32; 3]; 3])> {
        for (from_coord, to_coord_possible_set) in
            self.find_beacon_mappings_from(ref_beacons, ref_beacon_dists)
        {
            // to_coord is obtained from rotation followed by translation
            // so let's see what combos work

            let possible_transforms = to_coord_possible_set.iter().flat_map(|&to_coord| {
                axis_rotation_transforms().map(move |rot| {
                    let rotated = apply_transform(&rot, &from_coord);
                    let scanner_pos_rotated = vector_sub(&rotated, &to_coord);
                    (rot, scanner_pos_rotated)
                })
            });

            for (rot, scanner_pos_rotated) in possible_transforms {
                let mut ref_beacons_transf = ref_beacons
                    .iter()
                    .map(|beacon_pos| {
                        vector_sub(&apply_transform(&rot, beacon_pos), &scanner_pos_rotated)
                    })
                    .filter(|[x, y, z]| x.abs() < 1000 && y.abs() < 1000 && z.abs() < 1000);

                if ref_beacons_transf.all(|b| self.beacons.contains(&b)) {
                    let scanner_pos = apply_transform(&invert_3x3(&rot), &scanner_pos_rotated);
                    return Some((scanner_pos, rot));
                }
            }
        }

        None
    }

    fn find_beacon_mappings_from(
        &self,
        from: &HashSet<Coord>,
        from_beacon_dists: &BeaconDists,
    ) -> impl Iterator<Item = (Coord, HashSet<Coord>)> {
        // assume at first that all mappings are valid
        let mut possibilities: HashMap<Coord, HashSet<Coord>> = from
            .iter()
            .map(|&coords_from| {
                let coords_to: HashSet<Coord> = self.beacons.iter().copied().collect();
                (coords_from, coords_to)
            })
            .collect();

        // narrow down based on distances
        let to_beacon_dists = self.beacon_distances();
        for (from, to) in from_beacon_dists
            .iter()
            .filter(|&(dist, _)| to_beacon_dists.contains_key(dist))
            .map(|(dist, from)| (from, &to_beacon_dists[dist]))
        {
            let to_set = to
                .iter()
                .flat_map(|(to1, to2)| [*to1, *to2].into_iter())
                .collect();

            for (from1, from2) in from {
                *possibilities.get_mut(from1).unwrap() = possibilities[from1]
                    .intersection(&to_set)
                    .copied()
                    .collect();
                *possibilities.get_mut(from2).unwrap() = possibilities[from2]
                    .intersection(&to_set)
                    .copied()
                    .collect();
            }
        }

        // get rid of matchless items
        possibilities.retain(|_from, to_set| !to_set.is_empty());

        // put beacons with less matches first
        let mut possibilities = possibilities.into_iter().collect::<Vec<_>>();
        possibilities.sort_by_key(|(_from, to_set)| to_set.len());

        possibilities.into_iter()
    }
}

fn distance(c1: &Coord, c2: &Coord) -> i32 {
    c1.iter()
        .zip(c2.iter())
        .map(|(&p1, &p2)| (p1 - p2).abs())
        .sum()
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut current_scanner: Option<Scanner> = None;
    let mut id = 0_usize;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("--- scanner ") {
            if let Some(mut scanner) = current_scanner {
                // assign id
                scanner.id = id;
                id += 1;

                scanners.push(scanner);
                current_scanner = Some(Scanner::default());
            } else {
                // first run
                current_scanner = Some(Scanner::default());
            }
        } else if let Some(scanner) = &mut current_scanner {
            let beacon_coords = {
                let mut it = line.split(',').map(|s| s.parse::<i32>().unwrap());
                let coords = [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()];
                assert!(it.next().is_none());
                coords
            };
            scanner.beacons.insert(beacon_coords);
        } else {
            dbg!(line);
            unreachable!("input parse error");
        }
    }

    if let Some(mut scanner) = current_scanner {
        scanner.id = id;
        scanners.push(scanner);
    }

    scanners
}

fn calc_beacon_dists(beacons: &HashSet<Coord>, beacon_dists: &mut BeaconDists) {
    for (idx1, pos1) in beacons.iter().enumerate() {
        for pos2 in beacons.iter().skip(idx1 + 1) {
            let d = distance(pos1, pos2);
            //let MAX_DIST: i32 = (3.0 * (2000 as f64).powi(2)).sqrt().ceil() as i32; // should be const :/
            const MAX_DIST: i32 = 3465;

            if d <= MAX_DIST {
                beacon_dists
                    .entry(d)
                    .or_default()
                    .insert((pos1.to_owned(), pos2.to_owned()));
            }
        }
    }
}

fn apply_transform(transform: &[Coord; 3], from: &Coord) -> Coord {
    [
        transform[0].iter().zip(from).map(|(x, y)| x * y).sum(),
        transform[1].iter().zip(from).map(|(x, y)| x * y).sum(),
        transform[2].iter().zip(from).map(|(x, y)| x * y).sum(),
    ]
}

fn axis_rotation_transforms() -> impl Iterator<Item = [[i32; 3]; 3]> {
    // simplified discrete trigonometric functions
    #[inline]
    const fn sin(angle: i32) -> i32 {
        match angle {
            0 => 0,
            90 => 1,
            180 => 0,
            270 => -1,
            _ => unreachable!(),
        }
    }
    #[inline]
    const fn cos(angle: i32) -> i32 {
        match angle {
            0 => 1,
            90 => 0,
            180 => -1,
            270 => 0,
            _ => unreachable!(),
        }
    }

    const fn rotation_transform(alpha: i32, beta: i32, gamma: i32) -> [[i32; 3]; 3] {
        [
            [
                cos(alpha) * cos(beta),
                cos(alpha) * sin(beta) * sin(gamma) - sin(alpha) * cos(gamma),
                cos(alpha) * sin(beta) * cos(gamma) + sin(alpha) * sin(gamma),
            ],
            [
                sin(alpha) * cos(beta),
                sin(alpha) * sin(beta) * sin(gamma) + cos(alpha) * cos(gamma),
                sin(alpha) * sin(beta) * cos(gamma) - cos(alpha) * sin(gamma),
            ],
            [-sin(beta), cos(beta) * sin(gamma), cos(beta) * cos(gamma)],
        ]
    }

    let angles = || (0..4).map(|v| v * 90);
    angles()
        .cartesian_product(angles())
        .cartesian_product(angles())
        .map(|((alpha, beta), gamma)| rotation_transform(alpha, beta, gamma))
}

const fn invert_3x3(m: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    const fn determinant(m: &[[i32; 3]; 3]) -> i32 {
        m[0][0] * (m[1][1] * m[2][2] - m[2][1] * m[1][2])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    let d = determinant(m);
    [
        [
            (m[1][1] * m[2][2] - m[2][1] * m[1][2]) / d,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) / d,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / d,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) / d,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / d,
            (m[1][0] * m[0][2] - m[0][0] * m[1][2]) / d,
        ],
        [
            (m[1][0] * m[2][1] - m[2][0] * m[1][1]) / d,
            (m[2][0] * m[0][1] - m[0][0] * m[2][1]) / d,
            (m[0][0] * m[1][1] - m[1][0] * m[0][1]) / d,
        ],
    ]
}

const fn vector_sub(c1: &Coord, c2: &Coord) -> Coord {
    [c1[0] - c2[0], c1[1] - c2[1], c1[2] - c2[2]]
}

const fn vector_add(c1: &Coord, c2: &Coord) -> Coord {
    [c1[0] + c2[0], c1[1] + c2[1], c1[2] + c2[2]]
}

#[test]
fn transforms() {
    for transform in axis_rotation_transforms() {
        let v = [4, 5, 6];
        let inv_t = invert_3x3(&transform);

        assert_eq!(apply_transform(&inv_t, &apply_transform(&transform, &v)), v);
    }
}
