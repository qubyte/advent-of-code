use std::io::{self, stdin, Read};
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct FieldDefinition {
    name: String,
    range: Vec<usize>
}

fn parse_rules(line: &str) -> FieldDefinition {
    let name_val_split: Vec<String> = line.split(": ").map(String::from).collect();

    let name = name_val_split[0].clone();
    let mut whole_range = vec![];
    let range_split = name_val_split[1].split(" or ");

    for range in range_split {
        let start_stop: Vec<usize> = range.split('-').filter_map(|s| s.parse().ok()).collect();

        for i in start_stop[0]..=start_stop[1] {
            whole_range.push(i);
        }
    }

    FieldDefinition { name, range: whole_range }
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    stdin().read_to_string(&mut buffer)?;

    let split: Vec<String> = buffer.split("\n\n").map(String::from).collect();
    let field_definitions: Vec<FieldDefinition> = split[0].split('\n').map(parse_rules).collect();
    let my_ticket = parse_ticket(&split[1]);
    let nearby_tickets = split[2].trim().split('\n').map(parse_ticket);

    let valid_tickets: Vec<_> = nearby_tickets.filter(|ticket| {
        ticket.iter().all(|value| {
            field_definitions.iter().any(|rule| {
                rule.range.contains(value)
            })
        })
    }).collect();

    let mut assignments: Vec<_> = field_definitions.iter().map(|definition| {
        (0..field_definitions.len()).filter(|&index| {
            valid_tickets.iter().all(|ticket| {
                definition.range.contains(&ticket[index])
            })
        }).collect::<Vec<_>>()
    }).collect();

    let mut field_mapping = HashMap::new();

    loop {
        let index = assignments.iter().position(|v| v.len() == 1).unwrap();
        let val = assignments[index][0];

        field_mapping.insert(val, index);

        assignments = assignments.into_iter().map(|v| v.into_iter().filter(|&e| e != val).collect::<Vec<_>>()).collect::<Vec<_>>();

        if assignments.iter().all(|v| v.is_empty()) {
            break;
        }
    }

    let mut departure_prod = 1;

    for (ticket_index, field_index) in field_mapping {
        let field = &field_definitions[field_index];

        if field.name.starts_with("departure") {
            departure_prod *= my_ticket[ticket_index];
        }
    }

    println!("{}", departure_prod);

    Ok(())
}
