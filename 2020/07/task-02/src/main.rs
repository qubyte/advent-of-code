use recap::Recap;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

type Contents = HashMap<String, usize>;
type Bags = HashMap<String, Contents>;

#[derive(Deserialize, Recap)]
#[recap(regex = r"^(?P<color>-?[a-z ]+) bags contain (?P<contents>-?[a-z0-9 ,]+).")]
struct SemiParsedBag {
    color: String,
    contents: String,
}

struct Bag {
    color: String,
    contents: Contents,
}

fn complete_bag_parse(semi_parsed: SemiParsedBag) -> Bag {
    let mut contents: Contents = HashMap::new();

    if semi_parsed.contents != *"no other bags" {
        let re = Regex::new(r"(?P<number>\d+) (?P<color>[a-z ]+) bags?").unwrap();

        for captures in re.captures_iter(semi_parsed.contents.as_str()) {
            contents.insert(
                String::from(&captures["color"]),
                String::from(&captures["number"]).parse().unwrap(),
            );
        }
    }

    Bag {
        color: semi_parsed.color,
        contents,
    }
}

fn count_contents(color: &str, bags: &Bags) -> usize {
    let mut total = 0usize;

    if let Some(contents) = bags.get(color) {
        for (col, n) in contents {
            total += n + n * count_contents(col, bags);
        }
    }

    total
}

fn main() {
    let bags = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse::<SemiParsedBag>().ok())
        .map(complete_bag_parse)
        .fold(Bags::new(), |mut bags, bag| {
            bags.insert(bag.color, bag.contents);
            bags
        });

    println!("{}", count_contents(&String::from("shiny gold"), &bags));
}
