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
                let current = room[row * cols + col];

                if current == '.' {
                    next.push('.');
                    continue;
                }

                let mut occupied = 0;
                let is_first_row = row == 0;
                let is_last_row = row == (rows - 1);
                let is_first_col = col == 0;
                let is_last_col = col == (cols - 1);

                if !is_first_row {
                    if !is_first_col && room[(row - 1) * cols + col - 1] == '#' {
                        occupied += 1;
                    }

                    if room[(row - 1) * cols + col] == '#' {
                        occupied += 1;
                    }

                    if !is_last_col && room[(row - 1) * cols + col + 1] == '#' {
                        occupied += 1;
                    }
                }

                if !is_last_row {
                    if !is_first_col && room[(row + 1) * cols + col - 1] == '#' {
                        occupied += 1;
                    }

                    if room[(row + 1) * cols + col] == '#' {
                        occupied += 1;
                    }

                    if !is_last_col && room[(row + 1) * cols + col + 1] == '#' {
                        occupied += 1;
                    }
                }

                if !is_first_col && room[row * cols + col - 1] == '#' {
                    occupied += 1;
                }

                if !is_last_col && room[row * cols + (col + 1)] == '#' {
                    occupied += 1;
                }

                if current == 'L' {
                    if occupied == 0 {
                        next.push('#');
                    } else {
                        next.push('L');
                    }
                } else if occupied >= 4 {
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
