use std::env::args;
use std::collections::HashMap;

fn main() {
    let mut indices_of_number: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut last = 0;
    let mut len = 0;

    for (i, n) in args().skip(1).filter_map(|a| a.parse::<usize>().ok()).enumerate() {
        if let Some(v) = indices_of_number.get_mut(&n) {
            v.push(i);
        } else {
            indices_of_number.insert(n, vec![i]);
        }
        last = n;
        len += 1;
    }

    for i in len..30000000 {
        let collection = indices_of_number.get(&last).unwrap();
        let len = collection.len();

        let next = if len > 1 {
            i - collection[len - 2] - 1
        } else {
            0
        };

        if let Some(v) = indices_of_number.get_mut(&next) {
            v.push(i);
        } else {
            indices_of_number.insert(next, vec![i]);
        }

        last = next;
    }

    println!("{}", last);
}
