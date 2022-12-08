use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();

    let tree_grid: Vec<Vec<u8>> = input
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            line.chars()
                .filter_map(|c| format!("{}", c).parse::<u8>().ok())
                .collect()
        })
        .collect();

    let mut visibile: HashSet<(usize, usize)> = HashSet::new();
    let last_row_index = tree_grid.len() - 1;
    let last_col_index = tree_grid[0].len() - 1; // ASSUMPTION

    println!("test {:?}", (0..0).map(|i| tree_grid[1][i]).max());

    // Then iterate through the interior rows and columns ascending.
    for j in 0..=last_row_index {
        for i in 0..=last_col_index {
            let height = tree_grid[j][i];

            let is_visible = (0..i)
                .map(|i| tree_grid[j][i])
                .max()
                .map(|n| n < height)
                .unwrap_or(true)
                || (0..j)
                    .map(|j| tree_grid[j][i])
                    .max()
                    .map(|n| n < height)
                    .unwrap_or(true)
                || ((i + 1)..=last_col_index)
                    .rev()
                    .map(|i| tree_grid[j][i])
                    .max()
                    .map(|n| n < height)
                    .unwrap_or(true)
                || ((j + 1)..=last_row_index)
                    .rev()
                    .map(|j| tree_grid[j][i])
                    .max()
                    .map(|n| n < height)
                    .unwrap_or(true);

            if is_visible {
                visibile.insert((i, j));
            }
        }
    }

    for i in 0..=last_col_index {
        for j in 0..=last_row_index {
            print!("{}", if visibile.contains(&(i, j)) { '#' } else { ' ' });
        }
        println!("");
    }

    println!("{}", visibile.len());
}
