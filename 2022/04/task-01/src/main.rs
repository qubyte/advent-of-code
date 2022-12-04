use std::io::{stdin, BufRead};

fn parse_line(line: String) -> Option<((u32, u32), (u32, u32))> {
    let split: Result<[_; 4], _> = line
        .split(&['-', ','])
        .map(|n| n.parse::<u32>())
        .collect::<Vec<_>>()
        .try_into();

    match split {
        Ok([Ok(a), Ok(b), Ok(c), Ok(d)]) => Some(((a, b), (c, d))),
        _ => None
    }
}

fn is_inside(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
}

fn main() {
    let count = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(parse_line)
        .filter(|&(a, b)| is_inside(a, b))
        .count();

    println!("{}", count);
}
