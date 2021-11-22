use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    let total: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold(HashMap::new(), |mut register, line| {
            let mut instruction_set = line.split(',');
            let mask = instruction_set.next().unwrap();

            let (up, down) = mask.chars().rev().enumerate().fold((0usize, !0usize), |(up, down), (i, c)| {
                if c == '1' {
                    (up + (1 << i), down)
                } else if c == '0' {
                    (up, down - (1 << i))
                } else {
                    (up, down)
                }
            });

            instruction_set.for_each(|item| {
                let mut pair = item.split('=');
                let address = pair.next().unwrap().parse::<usize>().unwrap();
                let value = pair.next().unwrap().parse::<usize>().unwrap();

                register.insert(address, (value | up) & down);
            });

            register
        })
        .values()
        .sum();

    println!("{}", total);
}
