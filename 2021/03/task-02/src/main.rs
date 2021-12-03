use std::io::{stdin, BufRead};

type Input = Vec<usize>;

fn keep_most_common(input: Input, index: usize) -> Input {
    let (zero_rows, one_rows): (Input, Input) = input
        .into_iter()
        .partition(|&row| (row & (1 << index)) == 0);

    if zero_rows.len() > one_rows.len() {
        zero_rows
    } else {
        one_rows
    }
}

fn keep_least_common(input: Input, index: usize) -> Input {
    let (zero_rows, one_rows): (Input, Input) = input
        .into_iter()
        .partition(|&row| (row & (1 << index)) == 0);

    if zero_rows.len() > one_rows.len() {
        one_rows
    } else {
        zero_rows
    }
}

fn main() {
    // Cast lines of input directly to unsigned integers. Keep a record of the
    // largest value.
    let (input, max) = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|string| usize::from_str_radix(&string, 2).ok())
        .fold((vec![], 0), |(mut input, max), number| {
            input.push(number);
            (input, if number > max { number } else { max })
        });

    // The most significant (left-most) bit which will be 1 for some of the
    // input. Any higher and keep_least_common method will not work.
    let upper = (max as f64).log2() as usize;

    let mut oxygen_ratings = input.to_vec();

    // Most significant bits are on the left, so we have to count down.
    for i in (0..=upper).rev() {
        oxygen_ratings = keep_most_common(oxygen_ratings, i);

        if oxygen_ratings.len() == 1 {
            break;
        }
    }

    if oxygen_ratings.len() != 1 {
        panic!("There should be exactly one oxygen rating!");
    }

    let mut carbon_ratings = input.to_vec();

    for i in (0..=upper).rev() {
        carbon_ratings = keep_least_common(carbon_ratings, i);

        if carbon_ratings.len() == 1 {
            break;
        }
    }

    if carbon_ratings.len() != 1 {
        panic!("There should be exactly one carbon rating!");
    }

    println!("{}", oxygen_ratings[0] * carbon_ratings[0]);
}
