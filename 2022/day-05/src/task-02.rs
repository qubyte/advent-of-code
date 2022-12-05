use std::io::{stdin, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let split: Vec<_> = input.split("\n\n").collect();

    let mut state: Vec<Vec<char>> = vec![vec![]; 9];

    for line in split[0].lines().rev() {
        let chars: Vec<char> = line.chars().collect();

        for i in 0..state.len() {
            let input_index = 1 + i * 4;

            if chars[input_index].is_ascii_uppercase() {
                state[i].push(chars[input_index]);
            }
        }
    }

    for instruction in split[1].trim().split("\n") {
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
    }

    for column in state {
        print!("{}", column[column.len() - 1])
    }
    println!("");

    Ok(())
}
