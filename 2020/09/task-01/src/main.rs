use std::io::{stdin, BufRead};

fn main() {
    let preamble_length = 25;
    let numbers: Vec<usize> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    'outer: for i in preamble_length..numbers.len() {
        for j in (i - preamble_length)..(i - 1) {
            for k in (j + 1)..i {
                if numbers[i] == numbers[j] + numbers[k] {
                    continue 'outer;
                }
            }
        }

        println!("{}", numbers[i]);
        break;
    }
}
