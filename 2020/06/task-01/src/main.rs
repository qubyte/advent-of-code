use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let num: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            line.chars()
                .filter(|&c| c != ',')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!("{}", num);
}
