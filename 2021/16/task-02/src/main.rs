use std::io;

fn eval(input: &[char]) -> Result<(usize, usize), std::num::ParseIntError> {
    let type_id = usize::from_str_radix(&input[3..6].iter().collect::<String>(), 2)?;

    if type_id == 4 {
        return get_literal_and_length(&input);
    }

    let (literals, length) = parse_packets_for_operator(&input)?;

    let result = match type_id {
        0 => literals.into_iter().sum(),
        1 => literals.into_iter().product(),
        2 => *literals.iter().min().unwrap(),
        3 => *literals.iter().max().unwrap(),
        5 => if literals[0] > literals[1] { 1 } else { 0 },
        6 => if literals[0] < literals[1] { 1 } else { 0 },
        7 => if literals[0] == literals[1] { 1 } else { 0 },
        _ => panic!("Unknown type ID")
    };

    Ok((result, length))
}

fn parse_packets_for_operator(input: &[char]) -> Result<(Vec<usize>, usize), std::num::ParseIntError> {
    let mut packets = vec![];
    let mut next_offset;

    if input[6] == '0' { // Next 15 bits are the total length of subpackets.
        let length_bits = &input[7..(7 + 15)].iter().collect::<String>();
        let length = usize::from_str_radix(length_bits.as_str(), 2)?;
        let end_offset = 7 + 15 + length;

        next_offset = 7 + 15;

        while next_offset < end_offset {
            let (packet, length) = eval(&input[next_offset..])?;
            packets.push(packet);
            next_offset += length;
        }
    } else { // Next 11 bits are the number of subpackets.
        let n_subpackets_bits = input[7..(7 + 11)].iter().collect::<String>();
        let n_subpackets = usize::from_str_radix(n_subpackets_bits.as_str(), 2)?;

        next_offset = 7 + 11;

        for _ in 0..n_subpackets {
            let (packet, length) = eval(&input[next_offset..])?;
            packets.push(packet);
            next_offset += length;
        }
    }

    Ok((packets, next_offset))
}

fn get_literal_and_length(input: &[char]) -> Result<(usize, usize), std::num::ParseIntError> {
    // Residue from the second nibble is part of the first chunk of the number.
    let mut next_offset = 6;
    let mut literal_bits = vec![];

    loop {
        let is_last_chunk = input[next_offset] == '0';
        literal_bits.extend_from_slice(&input[(next_offset + 1)..(next_offset + 5)]);
        next_offset += 5;

        if is_last_chunk {
            let literal = usize::from_str_radix(literal_bits.iter().collect::<String>().as_str(), 2)?;
            break Ok((literal, next_offset));
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let input: Vec<_> = buffer.trim().chars().flat_map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Non-hex character.")
    }.chars()).collect();

    let (result, _) = eval(&input).unwrap_or((0, 0));

    println!("{}", result);

    Ok(())
}
