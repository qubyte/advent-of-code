use std::collections::HashSet;
use std::iter::repeat;
use std::io::{stdin, BufRead};

fn main() {
    let mut positions = vec![(0i32, 0i32); 10];
    let mut tail_positions = HashSet::from([(0i32, 0i32)]);

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .ok()
                .map(|r: [&str; 2]| r[1].parse().map(|c| (r[0], c)).ok())
                .flatten()
                .and_then(|(direction, count)| match direction {
                    "U" => Some((count, (1, 0))),
                    "D" => Some((count, (-1, 0))),
                    "R" => Some((count, (0, 1))),
                    "L" => Some((count, (0, -1))),
                    _ => None,
                })
        })
        .flat_map(|(count, unit_move)| repeat(unit_move).take(count))
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
                let x_too_far = x_diff.abs() == 2;
                let y_too_far = y_diff.abs() == 2;

                if x_too_far && y_too_far {
                    positions[position_index].0 += x_diff / 2;
                    positions[position_index].1 += y_diff / 2;
                } else if x_too_far {
                    positions[position_index].0 += x_diff / 2;
                    positions[position_index].1 = parent_y;
                } else if y_too_far {
                    positions[position_index].1 += y_diff / 2;
                    positions[position_index].0 = parent_x;
                } else {
                    // No movement in one position means no movements further
                    // down the rope.
                    return;
                }
            }

            tail_positions.insert(positions[positions.len() - 1].clone());
        });

    println!("{}", tail_positions.len());
}
