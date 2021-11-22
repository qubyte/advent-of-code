use std::io::{stdin, BufRead};

fn main() {
    let waiting_room: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| line.chars().collect())
        .collect();

    let rows = waiting_room.len();
    let cols = waiting_room[0].len();
    let mut room: Vec<char> = waiting_room.into_iter().flatten().collect();

    loop {
        let mut next = vec![];

        for row in 0..rows {
            for col in 0..cols {
                let loc = row * cols + col;
                let current = room[loc as usize];

                if current == '.' {
                    next.push('.');
                    continue;
                }

                let mut occupied = 0;

                for i in 1.. {
                    if i > row || i > col {
                        break;
                    }

                    let c = room[(row - i) * cols + col - i];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i > row {
                        break;
                    }

                    let c = room[(row - i) * cols + col];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i > row || i >= (cols - col) {
                        break;
                    }

                    let c = room[(row - i) * cols + col + i];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i >= (cols - col) {
                        break;
                    }

                    let c = room[row * cols + col + i];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i >= (rows - row) || i >= (cols - col) {
                        break;
                    }

                    let c = room[(row + i) * cols + col + i];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i >= (rows - row) {
                        break;
                    }

                    let c = room[(row + i) * cols + col];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i >= (rows - row) || i > col {
                        break;
                    }

                    let c = room[(row + i) * cols + col - i];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                for i in 1.. {
                    if i > col {
                        break;
                    }

                    let c = room[row * cols + col - i];

                    if c == '#' {
                        occupied += 1;
                        break;
                    } else if c == 'L' {
                        break;
                    }
                }

                if current == 'L' {
                    if occupied == 0 {
                        next.push('#');
                    } else {
                        next.push('L');
                    }
                } else if occupied >= 5 {
                    next.push('L');
                } else {
                    next.push('#');
                }
            }
        }

        if next == room {
            let total = next
                .into_iter()
                .fold(0, |total, loc| if loc == '#' { total + 1 } else { total });

            println!("{}", total);
            break;
        }

        room = next;
    }
}
