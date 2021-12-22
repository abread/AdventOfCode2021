struct Player {
    pos: u32,
    score: u32,
}

// aged like milk \/
trait Die {
    fn roll(&mut self) -> u32;
}

struct DeterministicDie {
    state: u32,
    roll_count: usize,
}

fn main() {
    let mut players = [Player::new(6), Player::new(4)];
    let mut det_die = DeterministicDie::default();

    let winner_idx = 'outer: loop {
        for (idx, p) in players.iter_mut().enumerate() {
            p.play(&mut det_die);

            if p.score >= 1000 {
                break 'outer idx
            }
        }
    };

    let loser = &players[(winner_idx + 1) % 2];
    dbg!(loser.score as usize * det_die.roll_count);
}

impl Player {
    fn new(starting_pos: u32) -> Self {
        Player {
            score: 0,
            pos: starting_pos,
        }
    }

    fn play(&mut self, die: &mut dyn Die) {
        for _ in 0..3 {
            self.pos = (self.pos - 1 + die.roll()) % 10 + 1;
        }

        self.score += self.pos;
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u32 {
        let n = self.state;
        self.state = (self.state + 1) % 100;

        self.roll_count += 1;

        n + 1
    }
}

#[allow(clippy::derivable_impls)]
impl Default for DeterministicDie {
    fn default() -> Self {
        DeterministicDie {
            state: 0,
            roll_count: 0,
        }
    }
}
