use std::io::{stdin, BufRead};

fn main() -> std::io::Result<()> {
    let mut state: Vec<Vec<char>> = vec![];
    let lines = stdin();

    loop {
        let mut line = String::new();
        lines.read_line(&mut line)?;

        if line.trim() == "" {
            break; // State initialized.
        }

        for (i, c) in line.chars().enumerate() {
            if i == 0 || i % 4 != 1 || !c.is_ascii_uppercase() {
                continue;
            }

            let state_index = (i - 1) / 4;

            while state.len() < state_index + 1 {
                state.push(vec![]);
            }

            state[state_index].push(c);
            state[state_index].rotate_right(1);
        }
    }

    lines
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|instruction| {
            if instruction.trim() == "" {
                return;
            }

            let chunks = instruction.split(' ').collect::<Vec<_>>();
            let count = chunks[1].parse::<usize>();
            let from = chunks[3].parse::<usize>();
            let to =  chunks[5].parse::<usize>();

            match (count, from, to) {
                (Ok(c), Ok(f), Ok(t)) => {
                    let len = state[f - 1].len() - c;
                    let mut removed: Vec<_> = state[f - 1].drain(len..).collect();
                    state[t - 1].append(&mut removed);
                },
                _ => ()
            }
        });

    for column in state {
        print!("{}", column[column.len() - 1])
    }
    println!("");

    Ok(())
}
