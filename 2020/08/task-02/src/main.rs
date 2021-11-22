use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Deserialize, Recap, Debug, Clone)]
#[recap(regex = r"^(?P<instruction>\w{3}) (?P<value>[\-+\d]+)")]
struct Operation {
    instruction: String,
    value: isize,
}

fn check_operations(operations: &[Operation]) -> Option<isize> {
    let mut accumulator = 0isize;
    let mut next = 0isize;
    let mut visited = HashSet::new();

    loop {
        if !visited.insert(next) {
            break None;
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
            break Some(accumulator);
        }
    }
}

fn main() {
    let (indices, operations) = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<Operation>().ok())
        .enumerate()
        .fold((vec![], vec![]), |(mut indices, mut ops), (i, op)| {
            if op.instruction == "jmp" || op.instruction == "nop" {
                indices.push(i);
            }

            ops.push(op);

            (indices, ops)
        });

    for i in indices {
        let mut updated = operations.clone();

        updated[i].instruction = if updated[i].instruction == "jmp" {
            String::from("nop")
        } else {
            String::from("jmp")
        };

        if let Some(accumulator) = check_operations(&updated) {
            println!("{}", accumulator);
            break;
        }
    }
}
