use std::io::{stdin, BufRead};

fn main() {
    let mut forward = 0isize;
    let mut depth = 0isize;
    let mut aim = 0isize;

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|line| {
            let split = line.split_whitespace();
            let collected: Vec<_> = split.collect();

            if collected.len() != 2 {
                return;
            }

            match collected[1].parse::<isize>() {
                Ok(amount) => match collected[0] {
                    "forward" => {
                        forward += amount;
                        depth += aim * amount;
                    },
                    "up" => aim -= amount,
                    "down" => aim += amount,
                    _ => ()
                }
                _ => ()
            };
        });

    println!("{}", forward * depth);
}
