use std::io::{stdin, BufRead};

fn main() {
    let instructions = stdin();

    let mut lines = instructions
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok());

    let estimate: usize = lines.next().unwrap().parse().unwrap();
    let (id, time_to_wait): (usize, usize) = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|id| id.parse::<usize>().ok())
        .map(|n| (n, n - (estimate % n)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!("{} {} {}", id, time_to_wait, id * time_to_wait);
}
