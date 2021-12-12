use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn main() {
    let mut grid: Vec<Vec<usize>> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            line.chars().map(|c| (c.to_string()).parse().unwrap()).collect()
        })
        .collect();

    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;
    let mut total = 0usize;

    for _step in 0..100 {
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                grid[row][col] += 1;
            }
        }

        let mut sparked: HashSet<(usize, usize)> = HashSet::new();

        loop {
            let mut count = 0usize;

            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    if grid[row][col] < 10 || sparked.contains(&(row, col)) {
                        continue;
                    }

                    sparked.insert((row, col));
                    count += 1;

                    let from_row = row.checked_sub(1).unwrap_or(row);
                    let from_col = col.checked_sub(1).unwrap_or(col);
                    let to_row = max_row.min(row + 1);
                    let to_col = max_col.min(col + 1);

                    for r in from_row..=to_row {
                        for c in from_col..=to_col {
                            grid[r][c] += 1;
                        }
                    }
                }
            }

            if count == 0 {
                break;
            }

            total += count;
        }

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] > 9 {
                    grid[row][col] = 0;
                }
            }
        }
    }

    println!("{}", total);
}
