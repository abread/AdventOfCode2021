use std::io::{self, BufRead};

#[derive(Debug)]
enum Instr {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_instr(s: impl AsRef<str>) -> Instr {
    let mut tokens = s.as_ref().split(' ');
    let typ = tokens.next().expect("instruction identifier");
    match typ {
        "forward" => {
            let n = tokens
                .next()
                .expect("forward accepts one argument")
                .parse()
                .expect("displacement is a positive integer");
            assert!(n > 0, "displacement is a positive integer");
            Instr::Forward(n)
        }
        "down" => {
            let n = tokens
                .next()
                .expect("forward accepts one argument")
                .parse()
                .expect("displacement is a positive integer");
            assert!(n > 0, "displacement is a positive integer");
            Instr::Down(n)
        }
        "up" => {
            let n = tokens
                .next()
                .expect("forward accepts one argument")
                .parse()
                .expect("displacement is a positive integer");
            assert!(n > 0, "displacement is a positive integer");
            Instr::Up(n)
        }
        _ => unreachable!("unknown instruction type"),
    }
}

fn main() {
    let (horiz, depth, _aim) = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(parse_instr)
        .fold(
            (0_i64, 0_i64, 0_i64),
            |(horiz, depth, aim), instr| match instr {
                Instr::Forward(n) => (horiz + n, depth + aim * n, aim),
                Instr::Up(n) => (horiz, depth, aim - n),
                Instr::Down(n) => (horiz, depth, aim + n),
            },
        );

    dbg!(horiz * depth);
}
