use std::io::{stdin, Read};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    for (index, group) in input.chars().collect::<Vec<_>>().windows(4).enumerate() {
        let deduplicated: HashSet<_> = group.iter().map(|&c| c).collect();

        if deduplicated.len() == 4 {
            println!("{}", index + 4);
            break;
        }
    }

    Ok(())
}
