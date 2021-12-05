use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let (draw_order, mut boards) = parse_input(&mut stdin.lock());

    let mut winners = Vec::new();

    for drawn_number in draw_order {
        if boards.is_empty() {
            // fast path
            break;
        }

        for b in &mut boards {
            b.draw(drawn_number);
        }

        // split winners
        let (winners_this_round, rest) = boards
            .into_iter()
            .partition::<Vec<_>, _>(BingoBoard::is_winner);

        // save winners, remove them from race
        if !winners_this_round.is_empty() {
            winners.push((drawn_number, winners_this_round));
        }
        boards = rest;
    }

    // part 1
    let (first_num, first_board) = winners
        .first()
        .map(|(drawn_num, winners)| (drawn_num, winners.first().unwrap()))
        .unwrap();
    dbg!(dbg!(first_num) * first_board.unmarked_numbers().sum::<u32>());

    // part 2
    let (last_num, last_board) = winners
        .last()
        .map(|(drawn_num, winners)| (drawn_num, winners.first().unwrap()))
        .unwrap();
    dbg!(dbg!(last_num) * last_board.unmarked_numbers().sum::<u32>());
}

#[derive(Default, Debug)]
struct BingoBoard([[u32; 5]; 5], [[bool; 5]; 5]);

impl BingoBoard {
    fn draw(&mut self, n: u32) {
        for (i, row) in self.0.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == n {
                    self.1[i][j] = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        let row_wins = self.1.iter().any(|row| row.iter().all(|&chosen| chosen));

        let col_wins = self
            .1
            .iter()
            .fold([true; 5], |mut state, row| {
                // compute AND across all columns at once
                for (idx, &is_chosen) in row.iter().enumerate() {
                    state[idx] = state[idx] && is_chosen;
                }

                state
            })
            .iter()
            .any(|&all_chosen| all_chosen);

        row_wins || col_wins
    }

    fn unmarked_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &val)| (i, j, val)))
            .filter(|(i, j, _)| !self.1[*i][*j])
            .map(|(_, _, val)| val)
    }
}

fn parse_input(input: &mut impl BufRead) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut draw_order = String::new();
    input.read_line(&mut draw_order).unwrap();
    let draw_order: Vec<u32> = draw_order
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut dummy_buf = String::new();
    assert_eq!(input.read_line(&mut dummy_buf).unwrap(), 1);
    assert_eq!(dummy_buf, "\n");

    let mut boards = Vec::new();
    while let Some(board) = parse_board(input) {
        boards.push(board);

        // read separating newline
        dummy_buf.clear();
        assert!(input.read_line(&mut dummy_buf).unwrap() <= 1);
        assert!(dummy_buf == "\n" || dummy_buf.is_empty());
    }

    (draw_order, boards)
}

fn parse_board(input: &mut impl BufRead) -> Option<BingoBoard> {
    let mut board = BingoBoard::default();

    for i in 0..5 {
        let mut line = String::new();
        if input.read_line(&mut line).unwrap() == 0 {
            assert!(i == 0);
            return None;
        }

        for (j, num_str) in line.split_ascii_whitespace().enumerate() {
            board.0[i][j] = num_str.parse().unwrap();
        }
    }

    Some(board)
}
