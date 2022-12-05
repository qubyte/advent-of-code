use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn byte_to_priority(b: u8) -> u32 {
    if b > 90 {
        b as u32 - 96
    } else {
        b as u32 - 38
    }
}

fn main() {
    let lines: Vec<_> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .collect();

    let total: u32 = lines
        .chunks(3)
        .filter_map(|triple| {
            triple
                .iter()
                .map(|bag| bag.as_bytes().into_iter().map(|&c| c).collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).map(|&c| c).collect())
        })
        .filter_map(|c| c.into_iter().last())
        .map(byte_to_priority)
        .sum();

    println!("{:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_byte() {
        assert_eq!(byte_to_priority(b'a'), 1);
    }

    #[test]
    fn uppercase_byte() {
        assert_eq!(byte_to_priority(b'A'), 27);
    }
}
