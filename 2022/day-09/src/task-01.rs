use std::collections::HashSet;
use std::io::{stdin, BufRead};

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let mut position_head = (0i32, 0i32);
    let mut position_tail = (0i32, 0i32);
    let mut tail_positions = HashSet::from([(0i32, 0i32)]);

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let split: Option<[_; 2]> = line.split_whitespace().collect::<Vec<_>>().try_into().ok();
            let parsed = split
                .map(|r| r[1].parse::<i32>().map(|c| (r[0], c)).ok())
                .flatten();

            match parsed {
                Some(("U", count)) => Some((Direction::Up, count)),
                Some(("D", count)) => Some((Direction::Down, count)),
                Some(("R", count)) => Some((Direction::Right, count)),
                Some(("L", count)) => Some((Direction::Left, count)),
                _ => None,
            }
        })
        .for_each(|(direction, count)| {
            for _ in 0..count {
                match direction {
                    Direction::Up => position_head.1 += 1,
                    Direction::Down => position_head.1 -= 1,
                    Direction::Right => position_head.0 += 1,
                    Direction::Left => position_head.0 -= 1,
                }

                let x_diff = position_head.0 - position_tail.0;
                let y_diff = position_head.1 - position_tail.1;

                if x_diff.abs() == 2 {
                    position_tail.0 += x_diff / 2;
                    if y_diff != 0 {
                        position_tail.1 += y_diff;
                    }
                }
                if y_diff.abs() == 2 {
                    position_tail.1 += y_diff / 2;
                    if x_diff != 0 {
                        position_tail.0 += x_diff;
                    }
                }

                tail_positions.insert(position_tail.clone());
            }
        });

    println!("{}", tail_positions.len());
}
