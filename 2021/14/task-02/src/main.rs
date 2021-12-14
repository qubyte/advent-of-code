use std::io::{stdin, BufRead};
use std::collections::HashMap;

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

    // This is a hash of characters to counts. It's initialized with the
    // characters in the template, and later updated with insertions.
    let mut insertions = template.iter().fold(HashMap::new(), |mut counts, &c| {
        *counts.entry(c).or_insert(0usize) += 1;
        counts
    });

    // A hash of counts for pairs of characters. These are all we need to work
    // out intertions per step. It'll be replaced on each step with updated
    // counts for pairs.
    let mut pairs = (1..template.len()).fold(HashMap::new(), |mut pairs, i| {
        *pairs.entry((template[i - 1], template[i])).or_insert(0usize) += 1;
        pairs
    });

    for _step in 0..40 {
        // This hash will replace pairs.
        let mut next = HashMap::new();

        for (pair, count) in pairs {
            let insertion = *rules.get(&pair).unwrap();
            let (left, right) = pair;

            // Based on the insertion rule, create two new pairs.
            let left_pair = (left, insertion);
            let right_pair = (insertion, right);

            // Populate the hash of pairs for the next step.
            *next.entry(left_pair).or_insert(0) += count;
            *next.entry(right_pair).or_insert(0) += count;

            // Increment insertions for the inserted character.
            *insertions.entry(insertion).or_insert(0) += count;
        }

        pairs = next;
    }

    let min = insertions.values().min().unwrap();
    let max = insertions.values().max().unwrap();

    println!("{}", max - min);
}
