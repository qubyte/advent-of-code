use std::io::{stdin, BufRead};

type Grid = Vec<Vec<Vec<bool>>>;
type HyperGrid = Vec<Grid>;

fn count(hyper_grid: &[Grid]) -> usize {
    hyper_grid.iter().fold(0usize, |count, grid| {
        count + grid.iter().fold(0, |count, layer| {
            count + layer.iter().fold(0, |count, row| {
                count + row.iter().fold(0, |count, el| if *el { count + 1 } else { count })
            })
        })
    })
}

fn step(hyper_grid: &[Grid]) -> HyperGrid {
    let hyper_depth = hyper_grid.len();
    let depth = hyper_grid[0].len();
    let height = hyper_grid[0][0].len();
    let width = hyper_grid[0][0][0].len();

    hyper_grid.iter().enumerate().map(|(l, grid)| {
        grid.iter().enumerate().map(|(k, layer)| {
            layer.iter().enumerate().map(|(j, row)| {
                row.iter().enumerate().map(|(i, &cube)| {
                    let width_range: Vec<_> = ((if i == 0 { i } else { i - 1 })..=(if i == (width - 1) { i } else { i + 1 })).collect();
                    let height_range: Vec<_> = ((if j == 0 { j } else { j - 1 })..=(if j == (height - 1) { j } else { j + 1 })).collect();
                    let depth_range: Vec<_> = ((if k == 0 { k } else { k - 1 })..=(if k == (depth - 1) { k } else { k + 1 })).collect();
                    let hyper_depth_range: Vec<_> = ((if l == 0 { l } else { l - 1 })..=(if l == (hyper_depth - 1) { l } else { l + 1 })).collect();

                    let mut total = 0;

                    for &l in &hyper_depth_range {
                        for &k in &depth_range {
                            for &j in &height_range {
                                for &i in &width_range {
                                    if hyper_grid[l][k][j][i] {
                                        total += 1;
                                    }
                                }
                            }
                        }
                    }

                    if cube {
                        total == 3 || total == 4
                    } else {
                        total == 3
                    }
                }).collect()
            }).collect()
        }).collect()
    }).collect()
}

fn build_full_hypergrid(initial_layer: &[Vec<bool>], steps: usize) -> HyperGrid {
    let initial_height = initial_layer.len();
    let initial_width = initial_layer[0].len();
    let initial_depth = 1;

    let height = initial_height + 2 * steps;
    let width = initial_width + 2 * steps;
    let depth = initial_depth + 2 * steps;
    let mut initial_layer: Vec<_> = initial_layer.iter().map(|row| {
        [vec![false; steps], row.clone(), vec![false; steps]].concat().to_vec()
    }).collect();

    for _ in 0..steps {
        initial_layer.insert(0, vec![false; width]);
        initial_layer.push(vec![false; width]);
    }

    let mut initial_grid = vec![initial_layer];
    let empty_layer: Vec<_> = (0..height).map(|_| vec![false; width]).collect();

    for _ in 0..steps {
        initial_grid.insert(0, empty_layer.clone());
        initial_grid.push(empty_layer.clone());
    }

    let mut hyper_grid = vec![initial_grid];

    for _ in 0..steps {
        let empty_grid: Vec<Vec<Vec<bool>>> = (0..depth).map(|_| empty_layer.clone()).collect();
        let another_empty_grid = empty_grid.clone();

        hyper_grid.insert(0, empty_grid);
        hyper_grid.push(another_empty_grid);
    }

    hyper_grid
}

fn main() {
    let initial_grid: Vec<Vec<bool>> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut hyper_grid = build_full_hypergrid(&initial_grid, 6);

    for _ in 0..6 {
        hyper_grid = step(&hyper_grid);
    }

    println!("{}", count(&hyper_grid));
}
