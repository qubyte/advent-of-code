use std::io::{stdin, BufRead};

struct Instruction {
    direction: char,
    value: isize,
}

fn main() {
    let instructions: Vec<Instruction> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap_or('x');

            if !String::from("NSEWLRF").contains(direction) {
                return None;
            }

            if let Ok(value) = chars.collect::<String>().parse::<isize>() {
                Some(Instruction { direction, value })
            } else {
                None
            }
        })
        .collect();

    let mut north_by = 0;
    let mut east_by = 0;
    let mut waypoint = [1isize, 10isize];

    for instruction in instructions {
        match instruction.direction {
            'L' => {
                waypoint = match instruction.value {
                    90 => [waypoint[1], -waypoint[0]],
                    180 => [-waypoint[0], -waypoint[1]],
                    270 => [-waypoint[1], waypoint[0]],
                    _ => waypoint,
                };
            }
            'R' => {
                waypoint = match instruction.value {
                    90 => [-waypoint[1], waypoint[0]],
                    180 => [-waypoint[0], -waypoint[1]],
                    270 => [waypoint[1], -waypoint[0]],
                    _ => waypoint,
                };
            }
            'F' => {
                north_by += waypoint[0] * instruction.value;
                east_by += waypoint[1] * instruction.value;
            }
            'N' => waypoint[0] += instruction.value,
            'S' => waypoint[0] -= instruction.value,
            'E' => waypoint[1] += instruction.value,
            'W' => waypoint[1] -= instruction.value,
            _ => panic!("Oh noes!"),
        }
    }

    println!("{}", north_by.abs() + east_by.abs());
}
