use recap::Recap;
use serde::Deserialize;
use std::io::{stdin, BufRead};

#[derive(Deserialize, Recap, Debug)]
#[recap(regex = r"^(?P<direction>[NSEWLRF])(?P<value>\d+)")]
struct Instruction {
    direction: char,
    value: isize,
}

fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

fn main() {
    let instructions: Vec<Instruction> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect();

    let mut facing = 1; // north = 0, east = 1, south = 2, west = 3
    let mut north_by = 0;
    let mut east_by = 0;

    for instruction in instructions {
        match instruction.direction {
            'L' => {
                facing = modulo(facing - (instruction.value / 90), 4);
            }
            'R' => {
                facing = modulo(facing + (instruction.value / 90), 4);
            }
            'F' => {
                if facing == 0 {
                    north_by += instruction.value;
                } else if facing == 1 {
                    east_by += instruction.value;
                } else if facing == 2 {
                    north_by -= instruction.value;
                } else {
                    east_by -= instruction.value;
                }
            }
            'N' => north_by += instruction.value,
            'S' => north_by -= instruction.value,
            'E' => east_by += instruction.value,
            'W' => east_by -= instruction.value,
            _ => panic!("Oh noes!"),
        }
    }

    println!("{}", north_by.abs() + east_by.abs());
}
