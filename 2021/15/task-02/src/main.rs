use std::io::{stdin, BufRead};
use std::collections::{HashMap, HashSet};

fn main() {
    let dangers_tile: Vec<Vec<usize>> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            line.chars().map(|c| c.to_string().parse().unwrap()).collect()
        })
        .collect();

    let tile_height = dangers_tile.len();
    let tile_width = dangers_tile[0].len();
    let mut dangers = vec![vec![0; 5 * tile_width]; 5 * tile_width];

    for j in 0..dangers.len() {
        for i in 0..dangers[0].len() {
            let mut danger = dangers_tile[j % tile_height][i % tile_width];

            for _ in 0..(j / tile_height + i / tile_width) {
                danger += 1;
                if danger == 10 {
                    danger = 1;
                }
            }

            dangers[j][i] = danger;
        }
    }

    let target = (dangers.len() - 1, dangers[dangers.len() - 1].len() - 1);

    let mut vertices = HashSet::new();
    let mut distances = HashMap::new();
    let mut previous = HashMap::new();

    for j in 0..dangers.len() {
        for i in 0..dangers[j].len() {
            distances.insert((i, j), usize::MAX);
            vertices.insert((i, j));
        }
    }

    distances.insert((0, 0), 0);

    while vertices.len() != 0 {
        let coord = vertices.iter().min_by(|a_coord, b_coord| {
            let a = distances.get(a_coord).unwrap();
            let b = distances.get(b_coord).unwrap();

            a.cmp(&b)
        }).unwrap();

        let i = coord.0;
        let j = coord.1;

        vertices.remove(&(i, j));

        if vertices.len() % 100 == 0 {
            println!("{}", vertices.len());
        }

        if (i, j) == target {
            break;
        }

        let this_dist = *distances.get_mut(&(i, j)).unwrap();

        if i > 0 {
            let d = this_dist + dangers[j][i - 1];

            if d < *distances.get(&(i - 1, j)).unwrap() {
                distances.insert((i - 1, j), d);
                previous.insert((i - 1, j), (i, j));
            }
        }
        if i < dangers[j].len() - 1 {
            let d = this_dist + dangers[j][i + 1];

            if d < *distances.get(&(i + 1, j)).unwrap() {
                distances.insert((i + 1, j), d);
                previous.insert((i + 1, j), (i, j));
            }
        }
        if j > 0 {
            let d = this_dist + dangers[j - 1][i];

            if d < *distances.get(&(i, j - 1)).unwrap() {
                distances.insert((i, j - 1), d);
                previous.insert((i, j - 1), (i, j));
            }
        }
        if j < dangers.len() - 1 {
            let d = this_dist + dangers[j + 1][i];

            if d < *distances.get(&(i, j + 1)).unwrap() {
                distances.insert((i, j + 1), d);
                previous.insert((i, j + 1), (i, j));
            }
        }
    }

    let mut u = target;
    let mut path = vec![u];

    loop {
        if let Some((i, j)) = previous.get(&u) {
            u = (*i, *j);
            path.push((*i, *j));
        } else {
            break;
        }
    }

    let mut total_danger = 0usize;

    for (i, j) in &path {
        total_danger += dangers[*j][*i];
    }

    println!("{}", total_danger - dangers[0][0]);
}
