use std::io::{self, stdin, Read};

struct FieldDefinition {
    range: Vec<usize>
}

fn parse_rules(line: &str) -> FieldDefinition {
    let name_val_split: Vec<String> = line.split(": ").map(String::from).collect();

    let mut whole_range = vec![];
    let range_split = name_val_split[1].split(" or ");

    for range in range_split {
        let start_stop: Vec<usize> = range.split('-').filter_map(|s| s.parse().ok()).collect();

        for i in start_stop[0]..=start_stop[1] {
            whole_range.push(i);
        }
    }

    FieldDefinition { range: whole_range }
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    stdin().read_to_string(&mut buffer)?;

    let split: Vec<String> = buffer.split("\n\n").map(String::from).collect();
    let rules: Vec<FieldDefinition> = split[0].split('\n').map(parse_rules).collect();
    // let my_ticket = parse_ticket(&split[1]);
    let nearby_tickets = split[2].split('\n').map(parse_ticket);

    let mut total = 0;

    for ticket in nearby_tickets {
        for value in ticket {
            if rules.iter().all(|rule| !rule.range.contains(&value)) {
                total+= value;
            }
        }
    }

    println!("{}", total);

    Ok(())
}
