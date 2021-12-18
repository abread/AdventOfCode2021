use std::ops::RangeInclusive;

fn main() {
    let start_pos = [0, 0];
    //let target_area = [20..=30, -10..=-5];
    let target_area = [14..=50, -267..=-225];
    let [v0x_range, v0y_range] = dbg!(velocity_limits(start_pos, &target_area));

    let mut combo_count = 0;
    let mut max_y = i64::MIN;
    for v0x in v0x_range {
        for v0y in v0y_range.clone() {
            // vy(t) = v0y - t
            // vy = 0 <=> t = v0y
            // y grows in t < v0y and decreases for t > v0y

            let mut x = start_pos[0];
            let mut y = start_pos[1];
            let mut vx = v0x;
            let mut vy = v0y;
            let mut tentative_max_y = i64::MIN;
            for t in 0_i64.. {
                x += vx;
                y += vy;
                vx -= vx.signum();
                vy -= 1;

                if y > tentative_max_y {
                    tentative_max_y = y;
                }

                if t > v0y && y < *target_area[1].start() {
                    // no target
                    break;
                }

                if target_area[0].contains(&x) && target_area[1].contains(&y) {
                    // reached target
                    combo_count += 1;

                    if tentative_max_y > max_y {
                        max_y = tentative_max_y;
                    }
                    break;
                }
            }
        }
    }

    dbg!(max_y);
    dbg!(combo_count);
}

fn velocity_limits(
    start_pos: [i64; 2],
    target_area: &[RangeInclusive<i64>; 2],
) -> [RangeInclusive<i64>; 2] {
    let v0x_range = v0x_limits(start_pos[0], &target_area[0]);
    let v0y_range = v0y_limits(start_pos[1], &target_area[1]);

    [v0x_range, v0y_range]
}

/*
discrete calculations mean only positions
0, v0x, 2v0x-1, 3v0x-3, ..., 1/2 v0x^2 + 1/2 v0x will ever be hit (same for y going up)
and the maximum height/length will be at t=v0x (for x, t=v0y for y)
sooo v0x <= xmax
*/

fn v0x_limits(start_pos: i64, target_range: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let min = if start_pos < *target_range.start() {
        // must have some speed to get to a proper position
        1
    } else {
        // we can go down to xmin at step 1
        i64::min(*target_range.start(), 0)
    };

    let max = if start_pos == *target_range.end() {
        0
    } else if start_pos <= *target_range.end() {
        *target_range.end()
    } else {
        unreachable!()
    };

    min..=max
}

fn v0y_limits(_start_pos: i64, target_range: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let min = {
        // we can go down to ymin at step 1
        i64::min(*target_range.start(), 0)
    };

    assert!(*target_range.start() < 0); // this reasoning assumes it
    let max = {
        // at t=2*v0y, y reaches 0 again
        // at t=2*v0y+1 y reaches -v0y-1
        // so we must ensure it doesn't go beyond ymin
        // -v0y-1 >= ymin <=> v0y <= -ymin-1
        -*target_range.start() - 1
    };
    min..=max
}
