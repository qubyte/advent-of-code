use std::io::{stdin, BufRead};

fn main() {
    let preamble_length = 25;
    let numbers: Vec<usize> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    let length = numbers.len();
    let mut not_sum = 0;

    'outer: for i in preamble_length..length {
        for j in (i - preamble_length)..(i - 1) {
            for k in (j + 1)..i {
                if numbers[i] == numbers[j] + numbers[k] {
                    continue 'outer;
                }
            }
        }

        not_sum = numbers[i];
        break;
    }

    for i in 0..length {
        let mut total = 0usize;
        let mut min = numbers[i];
        let mut max = numbers[i];

        for &num in numbers.iter().take(length).skip(i + 1) {
            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
            total += num;
            if total >= not_sum {
                break;
            }
        }

        if total == not_sum {
            println!("{}", min + max);
            break;
        }
    }
}
