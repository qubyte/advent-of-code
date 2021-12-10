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

    let mut scores: Vec<usize> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let mut stack = vec![];

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

                return None; // Syntax error.
            }

            let mut score = 0;

            for c in stack.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                }
            }

            Some(score)
        })
        .collect();

    scores.sort();

    println!("{}", scores[scores.len() / 2]);
}
