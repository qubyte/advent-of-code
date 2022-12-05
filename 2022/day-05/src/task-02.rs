use std::io::{stdin, BufRead};

fn main() -> std::io::Result<()> {
    let mut state: Vec<Vec<char>> = vec![];
    let input = stdin();

    input
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .take_while(|line| !line.trim().is_empty()) // An empty line means the end of the initializer.
        .for_each(|line| {
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
        });

    input
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|line| !line.trim().is_empty())
        .for_each(|instruction| {
            let chunks = instruction.split_whitespace().collect::<Vec<_>>();
            let count = chunks[1].parse::<usize>();
            let from = chunks[3].parse::<usize>();
            let to = chunks[5].parse::<usize>();

            match (count, from, to) {
                (Ok(c), Ok(f), Ok(t)) => {
                    let len = state[f - 1].len() - c;
                    let mut removed: Vec<_> = state[f - 1].drain(len..).collect();
                    state[t - 1].append(&mut removed);
                }
                _ => (),
            }
        });

    for column in state {
        print!("{}", column[column.len() - 1])
    }
    println!("");

    Ok(())
}
