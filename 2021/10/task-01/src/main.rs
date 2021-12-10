use std::io::{stdin, BufRead};
use std::collections::{HashSet, HashMap};

fn main() {
    let braces_map = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<')
    ]);

    let opening_braces: HashSet<char> = braces_map.clone().into_values().collect();
    let total: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let mut stack = vec![];
            let mut score = 0;

            for c in line.chars() {
                let last = stack.last().unwrap_or(&'x');

                if opening_braces.contains(&c) {
                    stack.push(c);
                    continue;
                }

                if let Some(opener) = braces_map.get(&c) {
                    if opener == last {
                        stack.pop();
                        continue;
                    }
                }

                score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0
                };

                break;
            }

            return score;
        })
        .sum();

    println!("{}", total);
}
