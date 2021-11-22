use std::io::{stdin, BufRead};

fn main() {
    let mut ids: Vec<_> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let binary: String = line
                .chars()
                .map(|c| if c == 'F' || c == 'L' { '0' } else { '1' })
                .collect();

            let (encoded_row, encoded_col) = binary.split_at(7);
            let row = usize::from_str_radix(encoded_row, 2).unwrap();
            let col = usize::from_str_radix(encoded_col, 2).unwrap();

            row * 8 + col
        })
        .collect();

    ids.sort_unstable();

    let least = ids[0];

    for (i, &id) in ids.iter().enumerate() {
        let expected = i + least;
        if id != expected {
            println!("{}", expected);
            break;
        }
    }
}
