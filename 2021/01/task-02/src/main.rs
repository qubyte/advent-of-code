use std::io::{stdin, BufRead};

fn main() {
    let mut window: Vec<u16> = vec![];
    let mut increases = 0u16;

    stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<u16>().ok())
        .for_each(|depth| {
            window.push(depth);

            if window.len() > 3 && depth > window.remove(0) {
                increases += 1;
            }
        });

    println!("{}", increases);
}
