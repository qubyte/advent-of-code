use recap::Recap;
use serde::Deserialize;
use std::io::{stdin, BufRead};

#[derive(Deserialize, Recap, Debug)]
#[recap(regex = r#"(?P<first>\d+)-(?P<second>\d+) (?P<letter>\w): (?P<password>\w+)"#)]
struct RuleAndPassword {
    first: usize,
    second: usize,
    letter: char,
    password: String,
}

fn main() {
    let total = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .filter_map(|rule_and_password: RuleAndPassword| {
            let password: Vec<_> = rule_and_password.password.chars().collect();
            let letter = rule_and_password.letter;
            let &first = password.get(rule_and_password.first - 1).unwrap();
            let &second = password.get(rule_and_password.second - 1).unwrap();

            if first == second {
                None
            } else if first == letter || second == letter {
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("{}", total);
}
