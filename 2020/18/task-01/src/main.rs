use std::io::{stdin, BufRead};
use std::iter;

fn evaluate(input_strings: &[String]) -> usize {
    let mut input = input_strings.to_owned();

    while let Some(i) = input.iter().position(|s| s == "+") {
        let sub_value =
            input[i - 1].parse::<usize>().unwrap() + input[i + 1].parse::<usize>().unwrap();

        input.splice((i - 1)..=(i + 1), iter::once(format!("{}", sub_value)));
    }

    input
        .into_iter()
        .filter(|item| item != "*")
        .filter_map(|item| item.parse::<usize>().ok())
        .product()
}

fn calculate(input_strings: &[String]) -> usize {
    let mut input = input_strings.to_owned();

    while let Some(i) = input.iter().position(|s| s == ")") {
        for j in (0..(i - 1)).rev() {
            if input[j] == "(" {
                let sub_value = evaluate(&input[(j + 1)..i].to_vec());
                input.splice(j..=i, iter::once(format!("{}", sub_value)));
                break;
            }
        }
    }

    evaluate(&input)
}

fn main() {
    let lines: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter(|&c| c != ' ')
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .map(|line| calculate(&line))
        .sum();

    println!("{}", lines);
}
