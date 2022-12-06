use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();

    let mut state: Vec<Vec<char>> = input
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .take_while(|line| !line.trim().is_empty()) // An empty line means the end of the initializer.
        .fold(vec![], |mut state, line| {
            for (i, c) in line.chars().enumerate() {
                if i % 4 == 1 && c.is_ascii_uppercase() {
                    let state_index = (i - 1) / 4;

                    while state.len() < state_index + 1 {
                        state.push(vec![]);
                    }

                    state[state_index].push(c);
                    state[state_index].rotate_right(1);
                }
            }

            state
        });

    input
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|line| !line.trim().is_empty())
        .filter_map(|instruction| {
            TryInto::<[usize; 3]>::try_into(
                instruction
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect::<Vec<_>>(),
            )
            .ok()
        })
        .for_each(|[count, from, to]| {
            let len = state[from - 1].len() - count;
            let mut removed = state[from - 1].drain(len..).collect();
            state[to - 1].append(&mut removed);
        });

    for column in state {
        print!("{}", column[column.len() - 1])
    }
    println!("");
}
