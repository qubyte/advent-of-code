use std::io::{stdin, BufRead};

enum Instruction {
    Noop,
    Addx,
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn main() {
    let mut clock = 0usize;
    let mut position = 1usize;
    let mut screen: Vec<bool> = vec![false; WIDTH * HEIGHT];

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            if line == "noop" {
                Some((Instruction::Noop, 0isize))
            } else {
                let result: Result<[&str; 2], _> =
                    line.split_whitespace().collect::<Vec<_>>().try_into();

                result.ok().and_then(|[instruction, amount]| {
                    if instruction != "addx" {
                        None
                    } else {
                        amount.parse::<isize>().ok().map(|n| (Instruction::Addx, n))
                    }
                })
            }
        })
        .for_each(|(instruction, amount)| {
            let mut tick = || {
                let row = clock % WIDTH;
                screen[clock] = screen[clock]
                    || (position == 0 && row == WIDTH - 1)
                    || (position == WIDTH - 1 && row == 0)
                    || (row as isize - position as isize).abs() < 2;
                clock = (clock + 1) % (WIDTH * HEIGHT);
            };

            match instruction {
                Instruction::Noop => {
                    tick();
                }
                Instruction::Addx => {
                    tick();
                    tick();
                    let iposition = (position as isize + amount) % (WIDTH as isize);
                    position = iposition as usize;
                }
            }
        });

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            print!("{}", if screen[j * WIDTH + i] { '#' } else { '.' });
        }
        println!("");
    }
}
