use std::io::{stdin, BufRead};

type Layer = Vec<Vec<bool>>;
type Grid = Vec<Layer>;

fn count(grid: &[Layer]) -> usize {
    grid.iter().fold(0usize, |count, layer| {
        count + layer.iter().fold(0, |count, row| {
            count + row.iter().fold(0, |count, el| if *el { count + 1 } else { count })
        })
    })
}

fn step(grid: &[Layer]) -> Grid {
    let mut padded_old = vec![];

    let depth = grid.len() + 2;
    let height = grid[0].len() + 2;
    let width = grid[0][0].len() + 2;

    let first_layer: Vec<_> = (0..height).map(|_| vec![false; width]).collect();
    let last_layer = first_layer.clone();

    padded_old.push(first_layer);
    let mut other_layers: Grid = grid.iter().map(|layer| {
        let other_rows: Layer = layer.iter().map(|row| {
            [vec![false; 1], row.clone(), vec![false; 1]].concat().to_vec()
        }).collect();

        [vec![vec![false; width]], other_rows, vec![vec![false; width]]].concat().to_vec()
    }).collect();
    padded_old.append(&mut other_layers);
    padded_old.push(last_layer);

    padded_old.iter().enumerate().map(|(k, layer)| {
        layer.iter().enumerate().map(|(j, row)| {
            row.iter().enumerate().map(|(i, &cube)| {
                let width_range: Vec<_> = ((if i == 0 { i } else { i - 1 })..=(if i == (width - 1) { i } else { i + 1 })).collect();
                let height_range: Vec<_> = ((if j == 0 { j } else { j - 1 })..=(if j == (height - 1) { j } else { j + 1 })).collect();
                let depth_range: Vec<_> = ((if k == 0 { k } else { k - 1 })..=(if k == (depth - 1) { k } else { k + 1 })).collect();

                let mut total = 0;

                for &k in &depth_range {
                    for &j in &height_range {
                        for &i in &width_range {
                            if padded_old[k][j][i] {
                                total += 1;
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
}

fn main() {
    let initial_grid: Layer = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut grid = vec![initial_grid];

    for _ in 0..6 {
        grid = step(&grid);
    }

    println!("{}", count(&grid));
}
