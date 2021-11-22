use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn many_worlds(mask: &str) -> Vec<(usize, usize)> {
    if let Some(i) = mask.find('X') {
        let down: String = mask.chars().enumerate().map(|(j, c)| if i == j { 'd' } else { c }).collect();
        let one: String = mask.chars().enumerate().map(|(j, c)| if i == j { '1' } else { c }).collect();
        [many_worlds(&down), many_worlds(&one)].concat()
    } else {
        vec![mask.chars().rev().enumerate().fold((0usize, !0usize), |(up, down), (i, c)| {
            if c == '1' {
                (up + (1 << i), down)
            } else if c == 'd' {
                (up, down - (1 << i))
            } else {
                (up, down)
            }
        })]
    }
}

fn main() {
    let total: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold(HashMap::new(), |mut register, line| {
            let mut instruction_set = line.split(',');
            let mask = String::from(instruction_set.next().unwrap());
            let mask_ups_and_downs = many_worlds(&mask);

            instruction_set.for_each(|item| {
                let mut pair = item.split('=');
                let address = pair.next().unwrap().parse::<usize>().unwrap();
                let value = pair.next().unwrap().parse::<usize>().unwrap();

                for (up, down) in &mask_ups_and_downs {
                    register.insert((address | *up) & *down, value);
                }
            });

            register
        })
        .values()
        .sum();

    println!("{}", total);
}
