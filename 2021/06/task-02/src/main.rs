use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let mut bins = [0usize; 9];

    buffer.trim().split(',').filter_map(|el| el.parse().ok()).for_each(|time: usize| {
        bins[time] += 1;
    });

    for _day in 0..256 {
        let new_fish = bins[0];

        for i in 1..9 {
            bins[i - 1] = bins[i];
        }

        bins[6] += new_fish;
        bins[8] = new_fish;
    }

    println!("{}", bins.iter().sum::<usize>());

    Ok(())
}
