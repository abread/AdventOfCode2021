use itertools::Itertools;

const N_PLAYERS: usize = 2;
const N_POS: usize = 10;
const MAX_SCORE: usize = 21;

fn main() {
    let mut win_counts = [[[[[None; N_POS]; N_POS]; MAX_SCORE + 1]; MAX_SCORE + 1]; N_PLAYERS];
    // to know counts for (score1, score2, pos1, pos2, next_player)
    // i must know counts for (score1+dp1, score2+dp2, pos1+r1+r2+r3 % 10, pos2+r4+r5+r6 % 10, (next_player + 1) % 2)

    win_counts[0][MAX_SCORE] = [[[Some((1_u128, 0_u128)); N_POS]; N_POS]; MAX_SCORE + 1];
    for counts in win_counts[1].iter_mut() {
        counts[MAX_SCORE] = [[Some((0, 1)); N_POS]; N_POS];
    }

    let states = scores_iter()
        .cartesian_product(0..N_PLAYERS)
        .cartesian_product(0..N_POS)
        .cartesian_product(0..N_POS)
        .map(|((((s1, s2), p), p1), p2)| (p, s1, s2, p1, p2));

    for (player, score1, score2, pos1, pos2) in states {
        win_counts[player][score1][score2][pos1][pos2] = dirac_3rolls()
            .map(|r123| {
                let mut next_score1 = score1;
                let mut next_score2 = score2;
                let mut next_pos1 = pos1;
                let mut next_pos2 = pos2;
                let next_player = (player + 1) % N_PLAYERS;

                if next_player == 0 {
                    next_pos1 = (pos1 + r123) % N_POS;
                    next_score1 = (score1 + next_pos1 + 1).min(MAX_SCORE);
                } else {
                    next_pos2 = (pos2 + r123) % N_POS;
                    next_score2 = (score2 + next_pos2 + 1).min(MAX_SCORE);
                }

                win_counts[next_player][next_score1][next_score2][next_pos1][next_pos2].unwrap()
            })
            .reduce(|(acc1, acc2), (x1, x2)| (acc1 + x1, acc2 + x2));
    }

    let (p1_winner_count, p2_winner_count) = win_counts[0][0][0][4][8].unwrap();
    dbg!(p1_winner_count);
    dbg!(p2_winner_count);
    dbg!(p1_winner_count.max(p2_winner_count));

    let (p1_winner_count, p2_winner_count) = win_counts[1][0][0][4][8].unwrap();
    dbg!(p1_winner_count);
    dbg!(p2_winner_count);
    dbg!(p1_winner_count.max(p2_winner_count));
}

fn dirac_3rolls() -> impl Iterator<Item = usize> + Clone {
    (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(move |((r1, r2), r3)| r1 + r2 + r3)
}

fn scores_iter() -> impl Iterator<Item = (usize, usize)> + Clone {
    let mut scores = (0..MAX_SCORE).rev()
        .cartesian_product((0..MAX_SCORE).rev())
        .collect_vec();
    scores.sort_unstable_by_key(|(s1, s2)| usize::MAX - *s1 - *s2);
    scores.into_iter()
}