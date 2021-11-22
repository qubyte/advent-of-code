use std::env::args;

fn main() {
    let mut input: Vec<usize> = args().skip(1).filter_map(|arg| arg.parse().ok()).collect();

    for _ in input.len()..2020 {
        let last = input[input.len() - 1];
        let position_before = input[..(input.len() - 1)].iter().rposition(|&n| n == last);

        if let Some(i) = position_before {
            input.push(input.len() - i - 1);
        } else {
            input.push(0);
        }
    }

    println!("{}", input[input.len() - 1]);
}
