use std::io::{stdin, BufRead};

fn main() {
    let public_keys: Vec<usize> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    let card_public_key = public_keys[0];
    let door_public_key = public_keys[1];

    let mut card_loop_size = 0usize;
    let mut value = 1usize;

    while value != card_public_key {
        card_loop_size += 1;
        value = (value * 7) % 20201227;
    }

    let mut encryption_key = 1usize;

    for _ in 0..card_loop_size {
        encryption_key = (encryption_key * door_public_key) % 20201227;
    }

    println!("encryption key: {}", encryption_key);
}
