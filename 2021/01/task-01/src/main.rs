use std::io::{stdin, BufRead};

fn main() {
    let mut last = std::u16::MAX;
    let mut increases = 0u16;

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<u16>().ok())
        .for_each(|depth| {
            if depth > last {
                increases += 1;
            }

            last = depth;
        });

    println!("{}", increases);
}
