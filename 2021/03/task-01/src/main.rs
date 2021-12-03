use std::io::{stdin, BufRead};

fn main() {
    let mut counts: Vec<isize> = vec![];

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|bin_string| {
            for (i, c) in bin_string.chars().enumerate() {
                if counts.len() < (i + 1) {
                    counts.push(0);
                }

                if c == '0' {
                    counts[i] -= 1;
                } else if c == '1' {
                    counts[i] += 1;
                }
            }
        });

    let mut bin_elements = String::new();
    let mut complement_elements = String::new();

    for element in counts.into_iter() {
        if element > 0 {
            bin_elements.push('1');
            complement_elements.push('0');
        } else {
            bin_elements.push('0');
            complement_elements.push('1');
        }
    }

    let result = usize::from_str_radix(&bin_elements, 2).unwrap();
    let complement = usize::from_str_radix(&complement_elements, 2).unwrap();

    println!("{}", result * complement);
}
