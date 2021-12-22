use std::io;

fn hex_char_to_nibble(c: char) -> Vec<char> {
    match c {
        '0' => vec!['0', '0', '0', '0'],
        '1' => vec!['0', '0', '0', '1'],
        '2' => vec!['0', '0', '1', '0'],
        '3' => vec!['0', '0', '1', '1'],
        '4' => vec!['0', '1', '0', '0'],
        '5' => vec!['0', '1', '0', '1'],
        '6' => vec!['0', '1', '1', '0'],
        '7' => vec!['0', '1', '1', '1'],
        '8' => vec!['1', '0', '0', '0'],
        '9' => vec!['1', '0', '0', '1'],
        'A' => vec!['1', '0', '1', '0'],
        'B' => vec!['1', '0', '1', '1'],
        'C' => vec!['1', '1', '0', '0'],
        'D' => vec!['1', '1', '0', '1'],
        'E' => vec!['1', '1', '1', '0'],
        'F' => vec!['1', '1', '1', '1'],
        _ => panic!("Non-hex character.")
    }
}

fn parse_packet(input: &Vec<char>, offset: usize) -> (usize, usize) {
    let version = usize::from_str_radix(&input[offset..(offset + 3)].iter().collect::<String>(), 2).unwrap();
    let type_id = usize::from_str_radix(&input[(offset + 3)..(offset + 6)].iter().collect::<String>(), 2).unwrap();

    if type_id == 4 {
        (version, get_offset_after_literal(&input, offset))
    } else {
        let (version_total, new_offset) = get_version_total_and_offset_after_operator(&input, offset);
        (version + version_total, new_offset)
    }
}

fn get_version_total_and_offset_after_operator(input: &Vec<char>, offset: usize) -> (usize, usize) {
    let mut versions_total = 0;
    let mut next_offset;

    if input[offset + 6] == '0' { // Next 15 bits are the total length of subpackets.
        let length_bits = &input[(offset + 7)..(offset + 7 + 15)].iter().collect::<String>();
        let length = usize::from_str_radix(length_bits.as_str(), 2).unwrap();
        let end_offset = offset + 7 + 15 + length;

        next_offset = offset + 7 + 15;

        while next_offset < end_offset {
            let result = parse_packet(&input, next_offset);
            versions_total += result.0;
            next_offset = result.1;
        }
    } else { // Next 11 bits are the number of subpackets.
        let n_subpackets_bits = input[(offset + 7)..(offset + 7 + 11)].iter().collect::<String>();
        let n_subpackets = usize::from_str_radix(n_subpackets_bits.as_str(), 2).unwrap();

        next_offset = offset + 7 + 11;

        for _ in 0..n_subpackets {
            let result = parse_packet(&input, next_offset);
            versions_total += result.0;
            next_offset = result.1;
        }
    }

    (versions_total, next_offset)
}

fn get_offset_after_literal(input: &Vec<char>, offset: usize) -> usize {
    // Residue from the second byte.
    let mut buffer = vec![input[offset + 6], input[offset + 7]];
    let mut next_offset = offset + 8;

    loop {
        let len = buffer.len();

        for _ in len..5 {
            buffer.push(input[next_offset]);
            next_offset += 1;
        }

        if buffer[0] == '0' {
            break next_offset;
        }

        buffer = vec![];
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let input: Vec<char> = buffer.trim().chars().flat_map(hex_char_to_nibble).collect();
    let (versions_total, _) = parse_packet(&input, 0);

    println!("{}", versions_total);

    Ok(())
}
