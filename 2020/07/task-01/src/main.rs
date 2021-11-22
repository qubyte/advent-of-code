use recap::Recap;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

#[derive(Deserialize, Recap, Debug)]
#[recap(regex = r"^(?P<color>-?[a-z ]+) bags contain (?P<contents>-?[a-z0-9 ,]+).")]
struct SemiParsedBag {
    color: String,
    contents: String,
}

#[derive(Debug)]
struct Bag {
    color: String,
    contents: HashSet<String>,
}

fn complete_bag_parse(semi_parsed: SemiParsedBag) -> Bag {
    let mut contents: HashSet<String> = HashSet::new();

    if semi_parsed.contents != *"no other bags" {
        let re = Regex::new(r"\d+ (?P<color>[a-z ]+) bags?").unwrap();

        for captures in re.captures_iter(semi_parsed.contents.as_str()) {
            contents.insert(String::from(&captures["color"]));
        }
    }

    Bag {
        color: semi_parsed.color,
        contents,
    }
}

type Bags = HashMap<String, HashSet<String>>;

fn contains_color(color: &str, bags: &Bags, containers: &mut HashSet<String>) {
    for (col, contents) in bags {
        if contents.contains(color) && !containers.contains(col) {
            containers.insert(col.clone());
            contains_color(col, bags, containers);
        }
    }
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
    let mut containers = HashSet::<String>::new();

    contains_color(&String::from("shiny gold"), &bags, &mut containers);

    println!("{:?}", containers.len());
}
