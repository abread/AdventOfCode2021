use bitflags::bitflags;
use itertools::Itertools;
use std::{
    io::{self, BufRead},
    ops::{BitAnd, Shr},
};

bitflags! {
    struct SegDisp: u8 {
        const TOP          = 0b00000001; // a
        const TOP_LEFT     = 0b00000010; // b
        const TOP_RIGHT    = 0b00000100; // c
        const MIDDLE       = 0b00001000; // d
        const BOTTOM_LEFT  = 0b00010000; // e
        const BOTTOM_RIGHT = 0b00100000; // f
        const BOTTOM       = 0b01000000; // g

        const ZERO = Self::TOP.bits | Self::TOP_LEFT.bits | Self::TOP_RIGHT.bits | Self::BOTTOM_LEFT.bits | Self::BOTTOM_RIGHT.bits | Self::BOTTOM.bits;

        const ONE = Self::TOP_RIGHT.bits | Self::BOTTOM_RIGHT.bits;

        const TWO = Self::TOP.bits | Self::TOP_RIGHT.bits | Self::MIDDLE.bits | Self::BOTTOM_LEFT.bits | Self::BOTTOM.bits;

        const THREE = Self::TOP.bits | Self::TOP_RIGHT.bits | Self::MIDDLE.bits | Self::BOTTOM_RIGHT.bits | Self::BOTTOM.bits;

        const FOUR = Self::TOP_LEFT.bits | Self::TOP_RIGHT.bits | Self::MIDDLE.bits | Self::BOTTOM_RIGHT.bits;

        const FIVE = Self::TOP.bits | Self::TOP_LEFT.bits | Self::MIDDLE.bits | Self::BOTTOM_RIGHT.bits | Self::BOTTOM.bits;

        const SIX = Self::TOP.bits | Self::TOP_LEFT.bits | Self::MIDDLE.bits | Self::BOTTOM_LEFT.bits | Self::BOTTOM_RIGHT.bits | Self::BOTTOM.bits;

        const SEVEN = Self::TOP.bits | Self::TOP_RIGHT.bits | Self::BOTTOM_RIGHT.bits;

        const EIGHT = Self::TOP.bits | Self::TOP_LEFT.bits | Self::TOP_RIGHT.bits | Self::MIDDLE.bits | Self::BOTTOM_LEFT.bits | Self::BOTTOM_RIGHT.bits | Self::BOTTOM.bits;

        const NINE = Self::TOP.bits | Self::TOP_LEFT.bits | Self::TOP_RIGHT.bits | Self::MIDDLE.bits | Self::BOTTOM_RIGHT.bits | Self::BOTTOM.bits;
    }
}

const NUMBERS: [SegDisp; 10] = [
    SegDisp::ZERO,
    SegDisp::ONE,
    SegDisp::TWO,
    SegDisp::THREE,
    SegDisp::FOUR,
    SegDisp::FIVE,
    SegDisp::SIX,
    SegDisp::SEVEN,
    SegDisp::EIGHT,
    SegDisp::NINE,
];

const SEGMENTS: [SegDisp; 7] = [
    SegDisp::TOP,
    SegDisp::TOP_LEFT,
    SegDisp::TOP_RIGHT,
    SegDisp::MIDDLE,
    SegDisp::BOTTOM_LEFT,
    SegDisp::BOTTOM_RIGHT,
    SegDisp::BOTTOM,
];
type WiringPermutation = [SegDisp; 7];

fn main() {
    let input = parse_input(io::stdin().lock());

    // part1
    dbg!(input
        .iter()
        .map(|(_training, eval)| eval.iter().filter(|&&s| confusion_score(s) == 1).count())
        .sum::<usize>());

    let mut sum_of_all = 0;
    for (training_set, eval_set) in &input {
        let permutation = find_inv_permutation(training_set);
        let number = eval_set
            .iter()
            .map(|&disp| apply_permutation(disp, permutation))
            .map(|disp| NUMBERS.iter().position(|&el| el == disp).unwrap())
            .fold(0, |state, x| state * 10 + x);

        sum_of_all += number;
    }

    dbg!(sum_of_all);
}

fn find_inv_permutation(training_set: &[SegDisp]) -> WiringPermutation {
    let mut possible_perms: [SegDisp; 7] = [SegDisp::EIGHT; 7];

    for &display in training_set {
        // get all the segments that should have been on for all number possibilities
        let possible = NUMBERS
            .iter()
            .copied()
            .filter(|&d| bit_count(d) == bit_count(display))
            .fold(SegDisp::empty(), SegDisp::union);

        let mangled_seg_indices: Vec<_> = iter_bits(display)
            .map(|b| SEGMENTS.iter().position(|&b2| b2 == b).unwrap())
            .collect();

        for &idx in &mangled_seg_indices {
            possible_perms[idx] = possible_perms[idx].intersection(possible);
        }
    }

    possible_perms
        .iter()
        .map(|&seg_poss| iter_bits(seg_poss).collect::<Vec<_>>())
        .multi_cartesian_product()
        .map(|v| v.try_into().unwrap())
        .filter(|&perm: &WiringPermutation| {
            // permutation must be a permutation (no two wires connected to the same thing)
            let mut conn_count = [0; 7];
            for idx in perm
                .iter()
                .copied()
                .flat_map(iter_bits)
                .map(|b| SEGMENTS.iter().position(|&b2| b == b2).unwrap())
            {
                conn_count[idx] += 1;
            }

            conn_count.into_iter().all(|c| c == 1)
        })
        .find(|&perm| {
            // only consider possibilities that actually work
            training_set
                .iter()
                .map(|&disp| apply_permutation(disp, perm))
                .all(|disp| NUMBERS.contains(&disp))
        })
        .unwrap()
}

fn iter_bits(disp: SegDisp) -> impl Iterator<Item = SegDisp> {
    SEGMENTS
        .iter()
        .copied()
        .filter(move |&seg| disp.contains(seg))
}

fn apply_permutation(disp: SegDisp, perm: WiringPermutation) -> SegDisp {
    let mut applied = SegDisp::empty();

    for seg_idx in iter_bits(disp).map(|bit| SEGMENTS.iter().position(|&b| b == bit).unwrap()) {
        applied = applied.union(perm[seg_idx]);
    }

    applied
}

fn confusion_score(number: SegDisp) -> u8 {
    // 0, 6, 9 (3 numbers) use 6 segments
    // 1 (1 number) uses 2 segments
    // 2, 3, 5 (3 numbers) use 5 segments
    // 4 (1 number) uses 4 segments
    // 7 (1 number) uses 3 segments
    // 8 (1 number) uses 7 segments

    match bit_count(number) {
        2 | 3 | 4 | 7 => 1,
        5 | 6 => 3,
        _ => u8::MAX,
    }
}

fn bit_count(number: SegDisp) -> usize {
    let mut bits = number.bits();
    let mut count = 0;
    while bits != 0 {
        if bits.bitand(0b1) == 0b1 {
            count += 1;
        }

        bits = bits.shr(1);
    }

    count
}

fn parse_input(reader: impl BufRead) -> Vec<(Vec<SegDisp>, Vec<SegDisp>)> {
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (training, evaluation) = line.split_once('|').unwrap();
            let training = parse_disps(training);
            let evaluation = parse_disps(evaluation);
            (training, evaluation)
        })
        .collect()
}

fn parse_disps(input: &str) -> Vec<SegDisp> {
    input
        .split(' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_disp)
        .collect()
}

fn parse_disp(input: impl AsRef<str>) -> SegDisp {
    let input = input.as_ref().trim();
    input
        .chars()
        .map(|c| match c {
            'a' => SegDisp::TOP,
            'b' => SegDisp::TOP_LEFT,
            'c' => SegDisp::TOP_RIGHT,
            'd' => SegDisp::MIDDLE,
            'e' => SegDisp::BOTTOM_LEFT,
            'f' => SegDisp::BOTTOM_RIGHT,
            'g' => SegDisp::BOTTOM,
            _ => unreachable!("segment identifiers are letters between a and g"),
        })
        .fold(SegDisp::empty(), |disp, x| disp.union(x))
}
