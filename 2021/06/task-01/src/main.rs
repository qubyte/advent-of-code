use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let mut input: Vec<usize> = buffer.trim().split(',').filter_map(|el| el.parse().ok()).collect();

    for _day in 0..80 {
        let mut new_fish = 0usize;

        for index in 0..input.len() {
            if input[index] == 0 {
                input[index] = 6;
                new_fish += 1;
            } else {
                input[index] -= 1;
            }
        }

        for _i in 0..new_fish {
            input.push(8);
        }
    }

    println!("{}", input.len());

    Ok(())
}
