use std::io::{stdin, BufRead};
use std::collections::HashSet;

type Coordinate = (usize, usize);

fn do_fold(line: &Coordinate, grid: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut new_grid = HashSet::new();
    let (fold_x, fold_y) = line;

    if *fold_x != 0 {
        for (x, y) in grid {
            if x < fold_x {
                new_grid.insert((*x, *y));
            } else if x > fold_x {
                new_grid.insert((2 * fold_x - x, *y));
            }
        }
    } else {
        for (x, y) in grid {
            if y < fold_y {
                new_grid.insert((*x, *y));
            } else if y > fold_y {
                new_grid.insert((*x, 2 * fold_y - y));
            }
        }
    }

    new_grid
}

fn main() {
    let mut done_reading_coordinates = false;

    let (mut coordinates, folds): (HashSet<Coordinate>, Vec<Coordinate>) = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold((HashSet::new(), vec![]), |(mut coordinates, mut folds), line| {
            if line == "" {
                done_reading_coordinates = true;
            } else if !done_reading_coordinates {
                let split: Vec<usize> = line.split(',').filter_map(|n| n.parse().ok()).collect();

                if split.len() == 2 {
                    coordinates.insert((split[0], split[1]));
                }
            } else {
                let split: Vec<_> = line.split_whitespace().collect();

                if split.len() == 3 {
                    let axis_and_loc: Vec<_> = split[2].split('=').collect();

                    if axis_and_loc.len() == 2 {
                        if let Ok(loc) = axis_and_loc[1].parse::<usize>() {
                            if axis_and_loc[0] == "x" {
                                folds.push((loc, 0));
                            } else if axis_and_loc[0] == "y" {
                                folds.push((0, loc));
                            }
                        }
                    }
                }
            }

            (coordinates, folds)
        });

    for fold in folds {
        coordinates = do_fold(&fold, &coordinates);
    }

    let x_max = coordinates.iter().map(|c| c.0).max().unwrap();
    let y_max = coordinates.iter().map(|c| c.1).max().unwrap();

    let mut output = vec![vec![' '; x_max + 1]; y_max + 1];

    for (x, y) in coordinates {
        output[y][x] = '#';
    }

    for line in &output {
        println!("{}", line.iter().collect::<String>());
    }
}
