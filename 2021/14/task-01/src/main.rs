use std::io::{stdin, BufRead};
use std::collections::{HashSet, HashMap};

fn main() {
    let (template, rules) = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold((vec![], HashMap::new()), |(template, mut rules), line| {
            if template.len() == 0 {
                return (line.chars().collect::<Vec<_>>(), rules);
            }
            if line.len() == 0 {
                return (template, rules);
            }

            let split: Vec<_> = line.split(" -> ").collect();

            if split.len() != 2 {
                return (template, rules);
            }

            let left: Vec<_> = split[0].chars().collect();

            if left.len() != 2 {
                return (template, rules);
            }

            let right: Vec<_> = split[1].chars().collect();

            if right.len() != 1 {
                return (template, rules);
            }

            rules.insert((left[0], left[1]), right[0]);

            (template, rules)
        });

    let mut polymer = template.to_vec();

    for _step in 0..10 {
        let mut next = vec![polymer[0]];

        for i in 1..polymer.len() {
            let left = polymer[i - 1];
            let right = polymer[i];

            if let Some(to_insert) = rules.get(&(left, right)) {
                next.push(*to_insert);
                next.push(right);
            }
        }

        polymer = next;
    }

    let unique: HashSet<_> = template.into_iter().collect();
    let mut counts = vec![];

    for c in unique {
        counts.push((c, polymer.iter().filter(|&&e| e == c).count()));
    }

    counts.sort_by(|&a, &b| a.1.cmp(&b.1));

    println!("{:?}", counts[counts.len() - 1].1 - counts[0].1);
}
