use std::io::{stdin, BufRead};

fn main() {
    let mut counts = [0usize; 10];

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|line| {
            let signals_and_outputs: Vec<_> = line.split(" | ").collect();

            if signals_and_outputs.len() != 2 {
                return;
            }

            signals_and_outputs[1].split_whitespace().for_each(|output| {
                match output.len() {
                    2 => counts[1] += 1,
                    3 => counts[7] += 1,
                    4 => counts[4] += 1,
                    7 => counts[8] += 1,
                    _ => ()
                };
            });
        });

    println!("{}", counts.iter().sum::<usize>());
}
