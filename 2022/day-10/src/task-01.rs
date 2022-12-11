use std::io::{stdin, BufRead};

enum Instruction {
    Noop,
    Addx,
}

const WANTED_TICKS: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    let mut clock = 0i32;
    let mut count = 1i32;
    let mut signal = 0i32;

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            if line == "noop" {
                Some((Instruction::Noop, 0i32))
            } else {
                let result: Result<[&str; 2], _> =
                    line.split_whitespace().collect::<Vec<_>>().try_into();

                result.ok().and_then(|[instruction, amount]| {
                    if instruction != "addx" {
                        None
                    } else {
                        amount.parse::<i32>().ok().map(|n| (Instruction::Addx, n))
                    }
                })
            }
        })
        .for_each(|(instruction, amount)| {
            let mut tick = || {
                clock += 1;
                if WANTED_TICKS.contains(&clock) {
                    signal += clock * count;
                }
            };

            match instruction {
                Instruction::Noop => {
                    tick();
                }
                Instruction::Addx => {
                    tick();
                    tick();
                    count += amount;
                }
            }
        });

    println!("{}", signal);
}
