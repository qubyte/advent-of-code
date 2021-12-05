use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    let mut points_map: HashMap<String, usize> = HashMap::new();
    let mut overlaps = 0usize;

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .for_each(|line| {
            let start_and_stop: Vec<_> = line.split(" -> ").collect();

            if start_and_stop.len() != 2 {
                return;
            }

            let start_nums: Vec<isize> = start_and_stop[0].split(',').filter_map(|s| s.parse().ok()).collect();

            if start_nums.len() != 2 {
                return;
            }

            let start_x = start_nums[0];
            let start_y = start_nums[1];

            let stop_nums: Vec<isize> = start_and_stop[1].split(',').filter_map(|s| s.parse().ok()).collect();

            if stop_nums.len() != 2 {
                return;
            }

            let stop_x = stop_nums[0];
            let stop_y = stop_nums[1];

            if start_x == stop_x {
                let mut min_max = vec![start_y, stop_y];
                min_max.sort();

                for y in min_max[0]..=min_max[1] {
                    let point = format!("{},{}", start_x, y);
                    let new_val = (*points_map.get(&point).unwrap_or(&0)) + 1;

                    points_map.insert(point, new_val);
                }
            } else if start_y == stop_y {
                let mut min_max = vec![start_x, stop_x];
                min_max.sort();

                for x in min_max[0]..=min_max[1] {
                    let point = format!("{},{}", x, start_y);
                    let new_val = (*points_map.get(&point).unwrap_or(&0)) + 1;

                    points_map.insert(point, new_val);
                }
            }
        });

    for (_key, val) in &points_map {
        if *val > 1 {
            overlaps += 1;
        }
    }

    println!("{}", overlaps);
}
