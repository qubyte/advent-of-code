use std::io::{stdin, BufRead};
use std::collections::HashSet;

type Caves = Vec<Vec<u8>>;
type Coordinate = (usize, usize);
type Basin = HashSet<Coordinate>;

fn crawl_basin(caves: &Caves, coord: &Coordinate, basin: &mut Basin) {
    if basin.contains(coord) {
        return;
    }

    let (i, j) = *coord;

    basin.insert((i, j));

    if i != 0 && caves[j][i - 1] != 9 {
        crawl_basin(&caves, &(i - 1, j), basin);
    }
    if i != caves[0].len() - 1 && caves[j][i + 1] != 9 {
        crawl_basin(&caves, &(i + 1, j), basin);
    }
    if j != 0 && caves[j - 1][i] != 9 {
        crawl_basin(&caves, &(i, j - 1), basin);
    }
    if j != caves.len() - 1 && caves[j + 1][i] != 9 {
        crawl_basin(&caves, &(i, j + 1), basin);
    }
}

fn main() {
    let caves: Caves = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            line.chars().map(|c| c.to_string().parse().unwrap()).collect()
        })
        .collect();

    let height = caves.len();
    let width = caves[0].len();
    let mut accounted_for: HashSet<(usize, usize)> = HashSet::new();
    let mut basin_sizes: Vec<usize> = vec![];

    for j in 0..height {
        for i in 0..width {
            let this = caves[j][i];

            if accounted_for.contains(&(i, j)) {
                // We've already assigned this to a basin, or ignored it.
                continue;
            }
            if this == 9 {
                // This coordinate is a ridge (not a basin). Account for it and
                // move on.
                accounted_for.insert((i, j));
                continue;
            }

            // If we got here, then we have a coordinate in a basin which hasn't
            // been accounted for. Crawl around to find the boundaries of its
            // basin.
            let mut basin: Basin = HashSet::new();

            crawl_basin(&caves, &(i, j), &mut basin);

            for &coord in &basin {
                accounted_for.insert(coord);
            }

            basin_sizes.push(basin.len());
        }
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));

    println!("{}", basin_sizes[..3].iter().product::<usize>());
}
