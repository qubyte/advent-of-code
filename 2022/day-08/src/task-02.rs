use std::io::{stdin, BufRead};

fn score(tree_grid: &Vec<Vec<u32>>, row: usize, column: usize) -> usize {
    let height = tree_grid[row][column];

    let mut count_left = 0;
    let mut count_right = 0;
    let mut count_up = 0;
    let mut count_down = 0;

    for i in (0..column).rev() {
        count_left += 1;
        if tree_grid[row][i] >= height {
            break;
        }
    }

    for i in (column + 1)..tree_grid[row].len() {
        count_right += 1;
        if tree_grid[row][i] >= height {
            break;
        }
    }

    for j in (0..row).rev() {
        count_up += 1;
        if tree_grid[j][column] >= height {
            break;
        }
    }

    for j in (row + 1)..tree_grid.len() {
        count_down += 1;
        if tree_grid[j][column] >= height {
            break;
        }
    }

    count_left * count_right * count_up * count_down
}

fn main() {
    let input = stdin();

    let tree_grid: Vec<Vec<u32>> = input
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let last_row_index = tree_grid.len() - 1;
    let last_col_index = tree_grid[0].len() - 1; // ASSUMPTION
    let mut best_score = 0usize;

    // Don't bother with the boundaries since they always score 0.
    for j in 1..last_row_index {
        for i in 1..last_col_index {
            let location_score = score(&tree_grid, j, i);

            if location_score > best_score {
                best_score = location_score;
            }
        }
    }

    println!("{}", best_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_grid(input: &str) -> Vec<Vec<u32>> {
        input
            .split('/')
            .map(|row| row.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect()
    }

    #[test]
    fn known_score_check_1() {
        let grid = prepare_grid("30373/25512/65332/33549/35390");
        assert_eq!(score(&grid, 3, 2), 8);
    }

    #[test]
    fn known_score_check_2() {
        let grid = prepare_grid("30373/25512/65332/33549/35390");
        assert_eq!(score(&grid, 1, 2), 4);
    }

    #[test]
    fn test_horizon() {
        let grid = prepare_grid("111/121/111");
        assert_eq!(score(&grid, 1, 1), 1);
    }

    #[test]
    fn test_boundary() {
        let grid = prepare_grid("111/121/111");
        assert_eq!(score(&grid, 0, 1), 0);
    }
}
