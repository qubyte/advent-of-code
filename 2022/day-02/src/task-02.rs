use std::io::{stdin, BufRead};

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

enum Counter {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn determine_score(opponent: Hand, outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Win => 6 + match opponent {
            Hand::Rock => Counter::Paper,
            Hand::Paper => Counter::Scissors,
            Hand::Scissors => Counter::Rock,
        } as u32,
        Outcome::Draw => 3 + match opponent {
            Hand::Rock => Counter::Rock,
            Hand::Paper => Counter::Paper,
            Hand::Scissors => Counter::Scissors,
        } as u32,
        Outcome::Loss => 0 + match opponent {
            Hand::Rock => Counter::Scissors,
            Hand::Paper => Counter::Rock,
            Hand::Scissors => Counter::Paper,
        } as u32
    }
}

fn main() {
    let score: u32 = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .map(|line| {
            let mut chars = line.chars();

            let hand = chars.next().and_then(|c| match c {
                'A' => Some(Hand::Rock),
                'B' => Some(Hand::Paper),
                'C' => Some(Hand::Scissors),
                _ => None,
            });

            let outcome = chars.last().and_then(|c| match c {
                'X' => Some(Outcome::Loss),
                'Y' => Some(Outcome::Draw),
                'Z' => Some(Outcome::Win),
                _ => None,
            });

            match (hand, outcome) {
                (Some(opponent_hand), Some(result)) => determine_score(opponent_hand, result),
                _ => 0
            }
        })
        .sum();

    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_win() {
        assert_eq!(determine_score(Hand::Rock, Outcome::Win), 2 + 6);
    }

    #[test]
    fn paper_win() {
        assert_eq!(determine_score(Hand::Paper, Outcome::Win), 3 + 6);
    }

    #[test]
    fn scissors_win() {
        assert_eq!(determine_score(Hand::Scissors, Outcome::Win), 1 + 6);
    }

    #[test]
    fn rock_draw() {
        assert_eq!(determine_score(Hand::Rock, Outcome::Draw), 1 + 3);
    }

    #[test]
    fn paper_draw() {
        assert_eq!(determine_score(Hand::Paper, Outcome::Draw), 2 + 3);
    }

    #[test]
    fn scissors_draw() {
        assert_eq!(determine_score(Hand::Paper, Outcome::Draw), 2 + 3);
    }

    #[test]
    fn rock_loss() {
        assert_eq!(determine_score(Hand::Rock, Outcome::Loss), 3 + 0);
    }

    #[test]
    fn paper_loss() {
        assert_eq!(determine_score(Hand::Paper, Outcome::Loss), 1 + 0);
    }

    #[test]
    fn scissors_loss() {
        assert_eq!(determine_score(Hand::Scissors, Outcome::Loss), 2 + 0);
    }
}
