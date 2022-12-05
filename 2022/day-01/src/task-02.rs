use std::io::{stdin, BufRead};

fn main() {
    let mut current = 0u32;
    let mut top_three = [0u32, 0u32, 0u32];

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|count| {
            if let Ok(n) = count.parse::<u32>() {
                current += n;
            } else {
                if current > top_three[0] {
                    top_three[2] = top_three[1];
                    top_three[1] = top_three[0];
                    top_three[0] = current;
                } else if current > top_three[1] {
                    top_three[2] = top_three[1];
                    top_three[1] = current;
                } else if current > top_three[2] {
                    top_three[2] = current;
                }
                current = 0;
            }
        });

    println!("{}", top_three.iter().sum::<u32>());
}
