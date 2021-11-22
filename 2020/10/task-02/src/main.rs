use std::io::{stdin, BufRead};

fn permute(group: &[u8]) -> Vec<Vec<u8>> {
    if group.len() == 1 {
        return vec![group.to_vec(), vec![]];
    }

    permute(&group[1..].to_vec())
        .into_iter()
        .flat_map(|smaller| {
            let mut bigger = vec![group[0]];
            bigger.extend(smaller.clone());

            vec![bigger, smaller]
        })
        .collect()
}

fn unwrap_and_permute(group: &[u8]) -> Vec<Vec<u8>> {
    let length = group.len();

    if group.len() < 3 {
        return vec![group.to_vec()];
    }

    let first = group[0];
    let last = group[length - 1];
    let middle = group[1..(length - 1)].to_vec();
    let middle_versions = permute(&middle);

    middle_versions
        .iter()
        .map(|group| {
            let mut wrapped = vec![first];
            wrapped.extend(group);
            wrapped.push(last);
            wrapped
        })
        .collect()
}

fn main() {
    let mut numbers: Vec<u8> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<u8>().ok())
        .collect();

    let mut groups = vec![vec![0]];
    let mut last = 0;
    let mut length = 1;

    numbers.sort_unstable();
    numbers.push(numbers[numbers.len() - 1] + 3);

    for number in numbers {
        if (number - last) < 3 {
            groups[length - 1].push(number);
        } else {
            groups.push(vec![number]);
            length += 1;
        }
        last = number;
    }

    let mut total = 1usize;

    for group in groups {
        total *= unwrap_and_permute(&group)
            .into_iter()
            .filter(|group| group.windows(2).all(|w| w[1] - w[0] <= 3))
            .count();
    }

    println!("{:?}", total)
}
