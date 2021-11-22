use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Deserialize, Recap, Debug)]
#[recap(regex = r"^(?P<instruction>\w{3}) (?P<value>[\-+\d]+)")]
struct Operation {
    instruction: String,
    value: isize,
}

fn main() {
    let operations: Vec<Operation> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<Operation>().ok())
        .collect();

    let mut accumulator = 0isize;
    let mut next = 0isize;
    let mut visited = HashSet::new();

    loop {
        if !visited.insert(next) {
            break;
        }

        if let Some(operation) = operations.get(next as usize) {
            match operation.instruction.as_str() {
                "nop" => next += 1,
                "acc" => {
                    next += 1;
                    accumulator += operation.value;
                }
                "jmp" => next += operation.value,
                _ => panic!("Should not reach here."),
            }
        } else {
            panic!("Operation not found.");
        }
    }

    println!("{}", accumulator);
}
