use std::io::{stdin, BufRead};

fn main() {
    let instructions = stdin();

    let mut lines = instructions
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok());
    lines.next();

    // t + n = id_x * a
    // t = id_x * a - n

    // (t + n) % id_x = 0
    // (t + m) % id_y = 0

    // (t + i) % id = 0
    // [(0, 67), (1, 7), (2, 59), (3, 61)]
    // t = 67 * a
    // t + 1 = 7 * b
    // t + 2 = 59 * c
    // t + 3 = 61 * d
    //
    // Where a, b, c, and d are all integers.
    // (67 * a + 1) / 7 = b, so (67 * a + 1) % 7 = 0
    // Counting upwards from 0, solutions for a are: 5 + 7n where n = 0, 1, 2...
    // (7 * b - 1) % 67 = 0: c are 48 + 67n
    //
    // t = 335 + 469n
    //
    // (67 * a + 2) % 59 = 0, a = 44 + 59n
    // (59 * c - 2) % 67 = 0, c = 37 + 67n
    //
    // t = 2948 + 3953m

    // [(0, 23), (13, 41), (23, 509), (36, 13), (37, 17), (52, 29), (54, 401), (60, 37), (73, 19)]

    // t = 23 * a
    // t + 23 = 509 * b
    // 23 * a + 23 = 509 * b
    // (23 * a + 23) % 509 = 0
    // a = 508, b = 23, t = 11684
    // a = 1017, b = 46,

    let ids: Vec<(usize, usize)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| {
            if let Ok(n) = id.parse() {
                Some((i, n))
            } else {
                None
            }
        })
        .collect();

    let (i, max_id) = ids.iter().max_by_key(|(_, id)| *id).unwrap();
    let filtered: Vec<(usize, usize)> = ids.clone().into_iter().filter(|(j, _)| *j != *i).collect();

    // Filtered is all the entries without the x's and the largest. The x's are
    // effectively 1 and will match all results. The largest is used to set the
    // initial offset and stepsize of the loop below, so it satisfies every
    // iteration too and is removed from the ids.

    // For all remaining buses, find the first time for which t + offset is
    // divisible by the id for all buses.
    // println!("{} {}", i, max_id);

    'outer: for t in ((*max_id - *i)..).step_by(*max_id) {
        for (offset, id) in &filtered {
            if (t + *offset) % *id != 0 {
                continue 'outer;
            }
        }

        println!("{}", t);
        break;
    }

    // let mut constraints = !vec[];

    // for i in (0..(ids.len() - 1)) {
    //     for j in (i..(ids.len())) {
    //         let (stopI, idI) = ids[i];
    //         let (stopJ, idJ) = ids[j];

    //         // t + stopI = idI * n
    //         // t + stopJ = idJ * m
    //         // (idI * n + - stopI + stopJ) % idJ = 0
    //         let mut n = 0;

    //         loop {
    //             n+=1;
    //             if (idI * n - stopI + stopJ) % idJ == 0 {
    //                 break;
    //             }
    //         }

    //         // t = idI * n - stopI
    //     }
    // }
}
