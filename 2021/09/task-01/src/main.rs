use std::io::{stdin, BufRead};

fn main() {
    let caves: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            line.chars().map(|c| format!("{}", c).parse().unwrap()).collect()
        })
        .collect();

    let height = caves.len();
    let width = caves[0].len();
    let mut risk = 0usize;

    for j in 0..height {
        for i in 0..width {
            let this = caves[j][i];
            if i != 0 && caves[j][i - 1] <= this {
                continue;
            }
            if j != 0 && caves[j - 1][i] <= this {
                continue;
            }
            if i != width - 1 && caves[j][i + 1] <= this {
                continue;
            }
            if j != height - 1 && caves[j + 1][i] <= this {
                continue;
            }
            risk += 1 + this as usize;
        }
    }

    println!("{}", risk);
}
