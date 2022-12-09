use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let mut positions = vec![(0i32, 0i32); 10];
    let mut tail_positions = HashSet::from([(0i32, 0i32)]);

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let split: Option<[_; 2]> = line.split_whitespace().collect::<Vec<_>>().try_into().ok();
            let parsed: Option<(&str, usize)> = split
                .map(|r| r[1].parse().map(|c| (r[0], c)).ok())
                .flatten();

            match parsed {
                Some(("U", count)) => Some((true, count, true)),
                Some(("D", count)) => Some((true, count, false)),
                Some(("R", count)) => Some((false, count, true)),
                Some(("L", count)) => Some((false, count, false)),
                _ => None,
            }
        })
        .flat_map(|(up, count, forwards)| {
            let sign = if forwards { 1 } else { -1 };

            std::iter::repeat(match up {
                true => (0i32, sign),
                false => (sign, 0i32),
            })
            .take(count)
        })
        .for_each(|(up, right)| {
            // Move the head.
            positions[0].0 += right;
            positions[0].1 += up;

            // Propagate movement down the rope.
            for position_index in 1..positions.len() {
                let parent_x = positions[position_index - 1].0;
                let parent_y = positions[position_index - 1].1;
                let x_diff = parent_x - positions[position_index].0;
                let y_diff = parent_y - positions[position_index].1;

                if x_diff.abs() == 2 {
                    positions[position_index].0 += x_diff / 2;
                    positions[position_index].1 = parent_y;
                } else if y_diff.abs() == 2 {
                    positions[position_index].1 += y_diff / 2;
                    positions[position_index].0 = parent_x;
                } else {
                    // No movement in one position means no movements further down the rope.
                    return;
                }
            }

            tail_positions.insert(positions[positions.len() - 1].clone());
        });

    println!("{}", tail_positions.len());
}

// incorrect tries
// 2097
