use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::str::FromStr;

use dashmap::DashSet;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Cuboid(
    RangeInclusive<i64>,
    RangeInclusive<i64>,
    RangeInclusive<i64>,
);

#[derive(Default)]
struct ReactorCore(DashSet<Cuboid>);

fn main() {
    let mut core = ReactorCore::default();

    for instr in io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<Instr>().unwrap())
    {
        //let region = Cuboid(-50..=50, -50..=50, -50..=50);
        let region = Cuboid(
            -i64::MAX..=i64::MAX,
            -i64::MAX..=i64::MAX,
            -i64::MAX..=i64::MAX,
        );

        println!("{:#?}", &instr);
        match instr {
            Instr::On(c) => {
                if let Some(c) = c.restrict(&region) {
                    core.on(c)
                }
            }
            Instr::Off(c) => {
                if let Some(c) = c.restrict(&region) {
                    core.off(c)
                }
            }
        }

        /*for p in core.sorted_on() {
            println!("{:?}", p);
        }*/
        println!("cuboid count: {}", core.0.len());
        dbg!(core.on_count());
        println!();
    }

    dbg!(core.on_count());
}

impl PartialOrd for Cuboid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cuboid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .start()
            .cmp(other.0.start())
            .then_with(|| self.0.end().cmp(other.0.end()))
            .then_with(|| self.1.start().cmp(other.1.start()))
            .then_with(|| self.1.end().cmp(other.1.end()))
            .then_with(|| self.2.start().cmp(other.2.start()))
            .then_with(|| self.2.end().cmp(other.2.end()))
    }
}

type CuboidBoxedIter = Box<dyn Iterator<Item = Cuboid> + Send>;
impl Cuboid {
    #[inline(always)]
    fn intersect(&self, other: &Cuboid) -> (CuboidBoxedIter, Option<Cuboid>, CuboidBoxedIter) {
        fn coord_intersection(
            range1: &RangeInclusive<i64>,
            range2: &RangeInclusive<i64>,
        ) -> Option<RangeInclusive<i64>> {
            if *range1.start() <= *range2.end() && *range2.start() <= *range1.end() {
                let start = *range1.start().max(range2.start());
                let end = *range1.end().min(range2.end());
                Some(start..=end)
            } else {
                None
            }
        }

        fn iter_exclusive_zone(
            c: &Cuboid,
            intersection: &Cuboid,
        ) -> Box<dyn Iterator<Item = Cuboid> + Send> {
            let y = intersection.1.clone();
            let z = intersection.2.clone();

            use std::iter::once;
            let it = once(if c.0.start() < intersection.0.start() {
                Some(Cuboid(
                    *c.0.start()..=*intersection.0.start() - 1,
                    y.clone(),
                    z.clone(),
                ))
            } else {
                None
            })
            .chain(once(if c.0.end() > intersection.0.end() {
                Some(Cuboid(*intersection.0.end() + 1..=*c.0.end(), y, z.clone()))
            } else {
                None
            }))
            .chain(once(if c.1.start() < intersection.1.start() {
                // left cover (constrained in height)
                Some(Cuboid(
                    c.0.clone(),
                    *c.1.start()..=*intersection.1.start() - 1,
                    z.clone(),
                ))
            } else {
                None
            }))
            .chain(once(if c.1.end() > intersection.1.end() {
                // right cover (constrained in height)
                Some(Cuboid(
                    c.0.clone(),
                    *intersection.1.end() + 1..=*c.1.end(),
                    z,
                ))
            } else {
                None
            }))
            .chain(once(if c.2.start() < intersection.2.start() {
                // bottom cover
                Some(Cuboid(
                    c.0.clone(),
                    c.1.clone(),
                    *c.2.start()..=*intersection.2.start() - 1,
                ))
            } else {
                None
            }))
            .chain(once(if c.2.end() > intersection.2.end() {
                // top cover
                Some(Cuboid(
                    c.0.clone(),
                    c.1.clone(),
                    *intersection.2.end() + 1..=*c.2.end(),
                ))
            } else {
                None
            }))
            .flatten();
            Box::new(it)
        }

        match (
            coord_intersection(&self.0, &other.0),
            coord_intersection(&self.1, &other.1),
            coord_intersection(&self.2, &other.2),
        ) {
            (Some(x), Some(y), Some(z)) => {
                let intersection = Cuboid(x, y, z);
                let self_bits = iter_exclusive_zone(self, &intersection);
                let other_bits = iter_exclusive_zone(other, &intersection);
                (self_bits, Some(intersection), other_bits)
            }
            _ => (
                Box::new(std::iter::once(self.clone())),
                None,
                Box::new(std::iter::once(other.clone())),
            ),
        }
    }

    fn restrict(self, restriction: &Cuboid) -> Option<Self> {
        self.intersect(restriction).1
    }
}

impl ReactorCore {
    fn on(&mut self, new_c: Cuboid) {
        let (removed, deduped_bits): (Vec<_>, Vec<_>) = self
            .0
            .par_iter()
            .filter_map(|c| {
                let c: &Cuboid = &*c;
                match c.intersect(&new_c) {
                    (c_bits, Some(intersection), new_bits) => {
                        Some((c.clone(), (c_bits, intersection, new_bits)))
                    }
                    _ => None,
                }
            })
            .unzip();

        if removed.is_empty() {
            // no conflicts, just add it
            self.0.insert(new_c);
            return;
        }

        removed.into_par_iter().for_each(|c| {
            self.0.remove(&c);
        });

        let semi_deduped_bits = deduped_bits
            .into_par_iter()
            .flat_map_iter(move |(c_bits, intersection, new_bits)| {
                c_bits.chain(new_bits).chain(std::iter::once(intersection))
            })
            .collect();

        fn dedup(mut set: DashSet<Cuboid>) -> DashSet<Cuboid> {
            let (to_remove, more_deduped): (Vec<_>, Vec<_>) = set
                .par_iter()
                .flat_map(|a| {
                    let a2 = a.clone();
                    set.par_iter()
                        .filter(move |b| **b > a2)
                        .map(move |b| (a.clone(), b.clone()))
                })
                .filter_map(|(a, b)| match a.intersect(&b) {
                    (a_bits, Some(intersection), b_bits) => {
                        Some(((a, b), (a_bits, intersection, b_bits)))
                    }
                    _ => None,
                })
                .unzip();

            // bah, had to allocate with unzip
            let more_deduped = more_deduped.into_par_iter()
                .flat_map_iter(|(a_bits, intersection, b_bits)| a_bits.chain(b_bits).chain(std::iter::once(intersection)))
                .collect::<DashSet<Cuboid>>();

            dbg!(to_remove.len());
            if to_remove.is_empty() {
                // was already deduped
                set
            } else {
                for (a, b) in to_remove {
                    set.remove(&a);
                    set.remove(&b);
                }

                set.extend(dedup(more_deduped).into_iter());
                set
            }
        }

        let deduped_bits = dedup(semi_deduped_bits);
        self.0.extend(deduped_bits.into_iter());
    }

    fn off(&mut self, to_remove: Cuboid) {
        let (to_remove, to_add): (Vec<_>, Vec<_>) = self.0
            .par_iter()
            .filter_map(|c| match c.intersect(&to_remove) {
                (to_add, Some(intersection_to_remove), _already_off) => Some(((c.clone(), intersection_to_remove), to_add)),
                _ => None,
            })
            .unzip();

        for (c, interesection) in to_remove {
            self.0.remove(&c);
            self.0.remove(&interesection);
        }

        for it in to_add {
            self.0.extend(it);
        }
    }

    fn on_count(&self) -> u128 {
        self.0
            .iter()
            .map(|c| {
                let x = *c.0.end() - *c.0.start() + 1;
                let y = *c.1.end() - *c.1.start() + 1;
                let z = *c.2.end() - *c.2.start() + 1;

                x as u128 * y as u128 * z as u128
            })
            .sum()
    }
}

#[derive(Debug)]
enum Instr {
    On(Cuboid),
    Off(Cuboid),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (tag, cuboid) = s.split_once(' ').unwrap();

        let mut dimension_iter = cuboid
            .split(',')
            .map(|d| d.split_once('=').unwrap().1)
            .map(|d| d.split_once("..").unwrap())
            .map(|(start, end)| (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()))
            .map(|(start, end)| start..=end);

        let cuboid = Cuboid(
            dimension_iter.next().unwrap(),
            dimension_iter.next().unwrap(),
            dimension_iter.next().unwrap(),
        );
        assert!(dimension_iter.next().is_none());

        Ok(match tag {
            "on" => Instr::On(cuboid),
            "off" => Instr::Off(cuboid),
            _ => unreachable!(),
        })
    }
}