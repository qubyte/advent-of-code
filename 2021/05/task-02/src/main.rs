use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    let overlaps: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold(HashMap::new(), |mut points_map, line| {
            let start_and_stop: Vec<_> = line.split(" -> ").collect();

            if start_and_stop.len() != 2 {
                return points_map;
            }

            let start_nums: Vec<isize> = start_and_stop[0].split(',').filter_map(|s| s.parse().ok()).collect();

            if start_nums.len() != 2 {
                return points_map;
            }

            let start_x = start_nums[0];
            let start_y = start_nums[1];

            let stop_nums: Vec<isize> = start_and_stop[1].split(',').filter_map(|s| s.parse().ok()).collect();

            if stop_nums.len() != 2 {
                return points_map;
            }

            let stop_x = stop_nums[0];
            let stop_y = stop_nums[1];

            if start_x == stop_x {
                // In this case we can't (and don't need to) calculate the
                // gradient of the line.
                let (y_min, y_max) = if start_y < stop_y { (start_y, stop_y) } else { (stop_y, start_y) };

                for y in y_min..=y_max {
                    let point = (start_x, y);
                    let new_val = (*points_map.get(&point).unwrap_or(&0)) + 1;

                    points_map.insert(point, new_val);
                }
            } else {
                // In the case of constant y or 45 degrees it's safe to
                // calculate m, even if it's not necessary for the former case.
                let m = (start_y - stop_y) / (start_x - stop_x);
                let c = start_y - m * start_x;
                let (x_min, x_max) = if start_x < stop_x { (start_x, stop_x) } else { (stop_x, start_x) };

                for x in x_min..=x_max {
                    let y = m * x + c;
                    let point = (x, y);
                    let new_val = (*points_map.get(&point).unwrap_or(&0)) + 1;

                    points_map.insert(point, new_val);
                }
            }

            points_map
        })
        .into_values()
        .map(|val| if val > 1 { 1 } else { 0 })
        .sum();

    println!("{}", overlaps);
}
