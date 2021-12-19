use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, separated_pair};
use nom::Finish;
use std::fmt::{Debug, Display};
use std::io::{self, BufRead};

#[derive(Clone, PartialEq)]
enum SNum {
    Lit(i64),
    Pair(Box<[SNum; 2]>),
}

fn main() {
    let sum = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<SNum>().unwrap())
        .reduce(|a, b| a.add(b))
        .unwrap();

    println!("{}", &sum);
    dbg!(sum.magnitude());
}

impl SNum {
    fn add(self, other: SNum) -> SNum {
        let mut n = SNum::Pair(Box::new([self, other]));
        n.reduce();
        n
    }

    fn reduce(&mut self) {
        while self.reduce_once().is_some() {
            // reduce until you can't reduce anymore
            println!(" {}", &self);
        }
    }

    fn reduce_once(&mut self) -> Option<()> {
        const MAX_EXPLOSION_DEPTH_DIFF: usize = 4;

        fn recurse(lvl1: [&mut SNum; 2], depth: usize) -> Option<([bool; 2], [i64; 2])> {
            let [left1, right1] = lvl1;
            match left1 {
                SNum::Pair(boxed_pair) => {
                    let [ref mut left2, ref mut right2] = **boxed_pair;

                    if left2.literal_mut().is_some()
                        && right2.literal_mut().is_some()
                        && depth >= MAX_EXPLOSION_DEPTH_DIFF
                    {
                        // begin explosion

                        let val_left = *left2.literal_mut().unwrap();
                        let val_right = *right2.literal_mut().unwrap();
                        *left1 = SNum::Lit(0);

                        let mut right_set = false;
                        if let Some(rval) = right1.first_lit_left() {
                            *rval += val_right;
                            right_set = true;
                        }

                        return Some(([false, right_set], [val_left, val_right]));
                    } else if let Some(([left_set, mut right_set], [val_left, val_right])) =
                        recurse([left2, right2], depth + 1)
                    {
                        if !right_set {
                            if let Some(rval) = right1.first_lit_left() {
                                *rval += val_right;
                                right_set = true;
                            }
                        }

                        return Some(([left_set, right_set], [val_left, val_right]));
                    }
                }
                SNum::Lit(n) => {
                    let n = *n;
                    if n >= 10 {
                        let rounded_down = n / 2;
                        let rounded_up = rounded_down + n % 2;

                        let rounded_down = SNum::Lit(rounded_down);
                        let rounded_up = SNum::Lit(rounded_up);
                        *left1 = SNum::Pair(Box::new([rounded_down, rounded_up]));
                        return Some(([true, true], [0, 0]));
                    }
                }
            }

            match right1 {
                SNum::Pair(boxed_pair) => {
                    let [ref mut left2, ref mut right2] = **boxed_pair;

                    if left2.literal_mut().is_some()
                        && right2.literal_mut().is_some()
                        && depth >= MAX_EXPLOSION_DEPTH_DIFF
                    {
                        // begin explosion

                        let val_left = *left2.literal_mut().unwrap();
                        let val_right = *right2.literal_mut().unwrap();
                        *right1 = SNum::Lit(0);

                        let mut left_set = false;
                        if let Some(lval) = left1.first_lit_right() {
                            *lval += val_left;
                            left_set = true;
                        }

                        return Some(([left_set, false], [val_left, val_right]));
                    } else if let Some(([mut left_set, right_set], [val_left, val_right])) =
                        recurse([left2, right2], depth + 1)
                    {
                        if !left_set {
                            if let Some(lval) = left1.first_lit_right() {
                                *lval += val_left;
                                left_set = true;
                            }
                        }

                        return Some(([left_set, right_set], [val_left, val_right]));
                    }
                }
                SNum::Lit(n) => {
                    let n = *n;
                    if n >= 10 {
                        let rounded_down = n / 2;
                        let rounded_up = rounded_down + n % 2;

                        let rounded_down = SNum::Lit(rounded_down);
                        let rounded_up = SNum::Lit(rounded_up);
                        *right1 = SNum::Pair(Box::new([rounded_down, rounded_up]));
                        return Some(([true, true], [0, 0]));
                    }
                }
            }

            None
        }

        match self {
            SNum::Pair(boxed_pair) => {
                let [ref mut left, ref mut right] = **boxed_pair;
                recurse([left, right], 1).map(|_| ())
            }
            SNum::Lit(n) => {
                let n = *n;
                if n >= 10 {
                    let rounded_down = n / 2;
                    let rounded_up = rounded_down + n % 2;

                    let rounded_down = SNum::Lit(rounded_down);
                    let rounded_up = SNum::Lit(rounded_up);
                    *self = SNum::Pair(Box::new([rounded_down, rounded_up]));
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn literal_mut(&mut self) -> Option<&mut i64> {
        match self {
            SNum::Lit(n) => Some(n),
            _ => None,
        }
    }

    fn first_lit_left(&mut self) -> Option<&mut i64> {
        match self {
            SNum::Lit(n) => Some(n),
            SNum::Pair(boxed_pair) => {
                let [ref mut left, ref mut right] = **boxed_pair;
                left.first_lit_left().or_else(|| right.first_lit_left())
            }
        }
    }

    fn first_lit_right(&mut self) -> Option<&mut i64> {
        match self {
            SNum::Lit(n) => Some(n),
            SNum::Pair(boxed_pair) => {
                let [ref mut left, ref mut right] = **boxed_pair;
                right.first_lit_right().or_else(|| left.first_lit_right())
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            &SNum::Lit(n) => n,
            SNum::Pair(boxed_pair) => {
                let [ref left, ref right] = **boxed_pair;
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }
}

impl std::str::FromStr for SNum {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::error::Error;
        match parse_snum(s).finish() {
            Ok((_rem, snum)) => Ok(snum),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_i64(s: &str) -> nom::IResult<&str, i64> {
    map_res(digit1, |input: &str| input.parse::<i64>())(s)
}

fn parse_snum(s: &str) -> nom::IResult<&str, SNum> {
    alt((
        map(parse_i64, SNum::Lit),
        map(
            delimited(
                tag("["),
                separated_pair(parse_snum, tag(","), parse_snum),
                tag("]"),
            ),
            |res| SNum::Pair(Box::new([res.0, res.1])),
        ),
    ))(s)
}

impl std::fmt::Debug for SNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(arg0) => Debug::fmt(arg0, f),
            Self::Pair(boxed_pair) => {
                let [ref left, ref right] = **boxed_pair;
                f.debug_list().entry(left).entry(right).finish()
            },
        }
    }
}

impl std::fmt::Display for SNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(arg0) => Display::fmt(arg0, f),
            Self::Pair(boxed_pair) => {
                let [ref left, ref right] = **boxed_pair;
                f.debug_list().entry(left).entry(right).finish()
            },
        }
    }
}