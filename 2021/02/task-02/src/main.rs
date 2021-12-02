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

            match (collected[0], collected[1].parse::<isize>()) {
                ("forward", Ok(amount)) => {
                    forward += amount;
                    depth += aim * amount;
                },
                ("up", Ok(amount)) => aim -= amount,
                ("down", Ok(amount)) => aim += amount,
                _ => (),
            }
        });

    println!("{}", forward * depth);
}
