use std::io::{stdin, BufRead};

fn main() {
    let mut numbers: Vec<usize> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    numbers.sort_unstable();

    let mut last = 0;
    let mut ones = 0;
    let mut threes = 1; // Include the last step, which we know is three.

    for n in numbers {
        let diff = n - last;
        last = n;

        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }

    println!("{}", ones * threes);
}
