use std::collections::{HashMap, HashSet};
use std::io;

fn count(chars: Vec<char>) -> HashSet<i32> {
    let mut matches = HashMap::new();

    for character in chars.iter() {
        matches
            .entry(character)
            .and_modify(|mat| *mat += 1)
            .or_insert(1);
    }

    let mut repeated_chars = HashSet::new();
    for (_, n) in matches {
        repeated_chars.insert(n);
    }

    repeated_chars
}

pub fn main(rows: Vec<Vec<char>>) -> io::Result<i32> {
    let mut repeated_chars = vec![];

    for chars in rows.iter() {
        repeated_chars.push(count(chars.to_vec()));
    }

    let mut count_2 = 0;
    let mut count_3 = 0;

    for set in repeated_chars.iter() {
        for n in set.iter() {
            if *n == 2 {
                count_2 += 1
            } else if *n == 3 {
                count_3 += 1
            }
        }
    }

    Ok(count_2 * count_3)
}
