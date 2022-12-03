use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn byte_to_priority(b: u8) -> u32 {
    if b > 90 {
        b as u32 - 96
    } else {
        b as u32 - 38
    }
}

fn main() {
    let sum: u32 = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let bytes = line.as_bytes();
            let len = bytes.len() / 2;
            let compartment_a: HashSet<_> = bytes[0..len].iter().collect();
            let compartment_b: HashSet<_> = bytes[len..].iter().collect();
            let item = compartment_a.intersection(&compartment_b);
            item.last().map(|&&b| byte_to_priority(b))
        })
        .sum();

    println!("{}", sum);
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
