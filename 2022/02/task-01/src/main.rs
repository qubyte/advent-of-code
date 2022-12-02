use std::io::{stdin, BufRead};

enum Hand {
    Rock,
    Paper,
    Scissors
}

fn determine_score(opponent: Hand, player: Hand) -> u32 {
    match player {
        Hand::Rock => 1 + match opponent {
            Hand::Rock => 3,
            Hand::Paper => 0,
            Hand::Scissors => 6
        },
        Hand::Paper => 2 + match opponent {
            Hand::Rock => 6,
            Hand::Paper => 3,
            Hand::Scissors => 0
        },
        Hand::Scissors => 3 + match opponent {
            Hand::Rock => 0,
            Hand::Paper => 6,
            Hand::Scissors => 3
        }
    }
}

fn main() {
    let score: u32 = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let mut chars = line.chars();

            let left = chars.next().map(|c| {
                match c {
                    'A' => Some(Hand::Rock),
                    'B' => Some(Hand::Paper),
                    'C' => Some(Hand::Scissors),
                    _ => None
                }
            }).flatten();

            let right = chars.last().map(|c| {
                match c {
                    'X' => Some(Hand::Rock),
                    'Y' => Some(Hand::Paper),
                    'Z' => Some(Hand::Scissors),
                    _ => None
                }
            }).flatten();

            if let Some(opponent) = left {
                if let Some(player) = right {
                    determine_score(opponent, player)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();

    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_rock() {
        assert_eq!(determine_score(Hand::Rock, Hand::Rock), 4);
    }

    #[test]
    fn rock_paper() {
        assert_eq!(determine_score(Hand::Rock, Hand::Paper), 8);
    }

    #[test]
    fn rock_scissors() {
        assert_eq!(determine_score(Hand::Rock, Hand::Scissors), 3);
    }

    #[test]
    fn paper_rock() {
        assert_eq!(determine_score(Hand::Paper, Hand::Rock), 1);
    }

    #[test]
    fn paper_paper() {
        assert_eq!(determine_score(Hand::Paper, Hand::Paper), 5);
    }

    #[test]
    fn paper_scissors() {
        assert_eq!(determine_score(Hand::Paper, Hand::Scissors), 9);
    }

    #[test]
    fn scissors_rock() {
        assert_eq!(determine_score(Hand::Scissors, Hand::Rock), 7);
    }

    #[test]
    fn scissors_paper() {
        assert_eq!(determine_score(Hand::Scissors, Hand::Paper), 2);
    }

    #[test]
    fn scissors_scissors() {
        assert_eq!(determine_score(Hand::Scissors, Hand::Scissors), 6);
    }
}
