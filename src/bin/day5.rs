use nom::{
    bytes::complete::tag, character::complete::i32 as i32_parser, combinator::map,
    sequence::separated_pair, Finish,
};
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut points = HashMap::new();
    for point in io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .flat_map(|input| CoordRange::parse(&input))
    {
        println!("{}, {}", point.0, point.1);
        *points.entry(point).or_insert(0_usize) += 1;
    }

    dbg!(points.values().filter(|&&num_lines| num_lines > 1).count());
}

type Coord = (i32, i32);

struct CoordRange {
    start: Coord,
    end: Coord,
    dx: i32,
    dy: i32,
    x_or_y: i32,
}

impl CoordRange {
    fn new(start: Coord, end: Coord) -> Self {
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        if (dx.abs() > dy.abs() && start.0 > end.0) || start.1 > end.1 {
            return CoordRange::new(end, start);
        }

        let x_or_y = if dx > dy { start.0 } else { start.1 };

        CoordRange {
            start,
            end,
            dx,
            dy,
            x_or_y,
        }
    }

    fn parse(input: &str) -> Self {
        let r: nom::IResult<&str, Self> = map(
            separated_pair(
                separated_pair(i32_parser, tag(","), i32_parser),
                tag(" -> "),
                separated_pair(i32_parser, tag(","), i32_parser),
            ),
            |(start, end)| CoordRange::new(start, end),
        )(input.trim());

        r.finish().unwrap().1
    }
}

impl Iterator for CoordRange {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        // Naive line drawing algorithm, applied upside down when |dx| <= |dy| (to avoid sparse lines)

        if self.dx.abs() > self.dy.abs() {
            let x = self.x_or_y;
            if x > self.end.0 {
                None
            } else {
                let y = self.start.1 + self.dy * (x - self.start.0) / self.dx;

                self.x_or_y += 1;
                Some((x, y))
            }
        } else {
            let y = self.x_or_y;
            if y > self.end.1 {
                None
            } else {
                let x = self.start.0 + self.dx * (y - self.start.1) / self.dy;

                self.x_or_y += 1;
                Some((x, y))
            }
        }
    }
}
