use std::io::{stdin, BufRead};
use std::collections::HashSet;

type Card = Vec<Vec<u8>>;

fn play_numbers_for_card(numbers: &Vec<u8>, card: &Card) -> Option<u32> {
    let rows = card.len();
    let cols = card[0].len();
    let mut winning = false;
    let mut score = 0u32;

    for i in 0..cols {
        let mut row_winner = true;

        for j in 0..rows {
            let cell = card[j][i];

            if !numbers.contains(&cell) {
                score += cell as u32;
                row_winner = false;
            }
        }

        if row_winner {
            winning = true;
        }
    }

    for j in 0..rows {
        let mut col_winner = true;

        for i in 0..cols {
            if !numbers.contains(&card[j][i]) {
                col_winner = false
            }
        }

        if col_winner {
            winning = true;
        }
    }

    if !winning {
        return None;
    }

    let last_number = *numbers.last().unwrap() as u32;

    Some(score * last_number)
}

fn main() {
    let (cards, numbers): (Vec<Card>, Vec<u8>) = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .fold((vec![], vec![]), |(mut input, numbers), row| {
            if row.len() == 0 {
                input.push(vec![]);
                (input, numbers)
            } else if numbers.len() == 0 {
                let numbers: Vec<u8> = row.split(',').filter_map(|el| el.parse().ok()).collect();
                (input, numbers)
            } else {
                if let Some(last) = input.last_mut() {
                    let bingo_row: Vec<u8> = row.split_whitespace().filter_map(|el| el.parse::<u8>().ok()).collect();
                    last.push(bingo_row);
                }
                (input, numbers)
            }
        });

    let mut round: Vec<u8> = vec![];
    let mut winners: HashSet<&Card> = HashSet::new();

    for i in 0..numbers.len() {
        round.push(numbers[i]);

        let remaining: Vec<&Card> = cards.iter().filter(|&card| !winners.contains(card)).collect();

        for card in remaining {
            if let Some(score) = play_numbers_for_card(&round, card) {
                winners.insert(card);
                if winners.len() == cards.len() {
                    println!("{}", score);
                }
            }
        }
    }
}
