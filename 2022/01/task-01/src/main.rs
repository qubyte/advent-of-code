use std::io::{stdin, BufRead};

fn main() {
    let mut current = 0u32;
    let mut max = 0u32;

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|count| {
            if let Ok(n) = count.parse::<u32>() {
                current += n;
            } else {
                if current > max {
                    max = current;
                }
                current = 0;
            }
        });

    println!("{}", max);
}
