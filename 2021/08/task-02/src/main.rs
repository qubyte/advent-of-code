use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn main() {
    let total: usize = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| {
            let signals_and_outputs: Vec<_> = line.split(" | ").collect();

            if signals_and_outputs.len() != 2 {
                return None;
            }

            let signals: Vec<_> = signals_and_outputs[0].split_whitespace().collect();

            let mut encoded: Vec<HashSet<char>> = vec![
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()
            ];

            // We can know the characters for 1, 4, 7, and 8 just by their
            // lengths.
            for s in &signals {
                match s.len() {
                    2 => {
                        for c in s.chars() {
                            encoded[1].insert(c);
                        }
                    },
                    3 => {
                        for c in s.chars() {
                            encoded[7].insert(c);
                        }
                    },
                    4 => {
                        for c in s.chars() {
                            encoded[4].insert(c);
                        }
                    },
                    7 => {
                        for c in s.chars() {
                            encoded[8].insert(c);
                        }
                    },
                    _ => ()
                }
            }

            // COMPOSE 0, 6, 9.

            for &s in &signals {
                if s.len() != 6 {
                    continue;
                }

                // There are three numbers which use six characters. The only one
                // which shares all the characters of 4 is 9. The others are 0, 6.
                if encoded[4].iter().all(|&c| s.contains(c)) {
                    for c in s.chars() {
                        encoded[9].insert(c);
                    }
                    continue;
                }

                // 6 is the only 6 character number which doesn't contain all the
                // characters of 1.
                if !encoded[1].iter().all(|&c| s.contains(c)) {
                    for c in s.chars() {
                        encoded[6].insert(c);
                    }
                    continue;
                }

                // The last six number character is 0.
                for c in s.chars() {
                    encoded[0].insert(c);
                }
            }

            // COMPOSE 3, 5, 2.

            for &s in &signals {
                if s.len() != 5 {
                    continue;
                }

                // 3 shares characters with 1, whereas 2 and 5 do not.encoded
                if encoded[1].iter().all(|&c| s.contains(c)) {
                    for c in s.chars() {
                        encoded[3].insert(c);
                    }
                    continue;
                }

                // 5 shares characters with 6 whereas 2 does not.
                if s.chars().all(|c| encoded[6].contains(&c)) {
                    for c in s.chars() {
                        encoded[5].insert(c);
                    }
                    continue;
                }

                // The remaining 5 character number is 2.
                for c in s.chars() {
                    encoded[2].insert(c);
                }
            }

            signals_and_outputs[1].split_whitespace().map(|s| {
                for i in 0..encoded.len() {
                    if encoded[i].len() == s.len() && s.chars().all(|c| encoded[i].contains(&c)) {
                        return Some(format!("{}", i));
                    }
                }
                None
            }).collect()
        })
        .filter_map(|output: String| output.parse::<usize>().ok())
        .sum();

    println!("{}", total);
}
