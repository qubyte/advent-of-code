use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let num: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let passengers: Vec<_> = line
                .split(',')
                .map(|passenger| passenger.chars().collect::<HashSet<char>>())
                .collect();

            passengers[0]
                .iter()
                .filter(|&c| passengers[1..].iter().all(|p| p.contains(c)))
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!("{}", num);
}
