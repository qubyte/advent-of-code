use std::io;

fn calculate_expendature(crab_subs: &Vec<usize>, to_loc: usize) -> usize {
    crab_subs.iter().fold(0, |fuel, &sub| {
        let moves = if sub > to_loc { sub - to_loc } else { to_loc - sub };
        fuel + (moves * (moves + 1)) / 2
    })
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let crab_subs: Vec<usize> = buffer.trim().split(',').filter_map(|el| el.parse().ok()).collect();
    let mean_float = crab_subs.iter().sum::<usize>() as f32 / crab_subs.len() as f32;
    let expended_ceil = calculate_expendature(&crab_subs, mean_float.ceil() as usize);
    let expended_floor = calculate_expendature(&crab_subs, mean_float.floor() as usize);
    let expended = expended_ceil.min(expended_floor);

    println!("{}", expended);

    Ok(())
}
