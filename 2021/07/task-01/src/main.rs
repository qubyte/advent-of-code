use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let mut crab_subs: Vec<usize> = buffer.trim().split(',').filter_map(|el| el.parse().ok()).collect();
    crab_subs.sort();
    let index = crab_subs.len() / 2;
    let median = crab_subs[index];
    let moves = crab_subs.iter().fold(0usize, |moves, &sub| {
        moves + if sub > median { sub - median } else { median - sub }
    });

    println!("{}", moves);

    Ok(())
}
