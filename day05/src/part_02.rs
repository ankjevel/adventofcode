use std::io;

fn remaining(units: &Vec<char>) -> Vec<char> {
    let mut remaining: Vec<char> = vec![];

    units.iter().for_each(|a| {
        if let Some(b) = remaining.pop() {
            if a != &b && a.to_ascii_uppercase() == b.to_ascii_uppercase() {
                return;
            }

            remaining.push(b);
        }
        remaining.push(*a);
    });

    remaining
}

pub fn main(units: &Vec<char>) -> io::Result<u32> {
    let chars = remaining(&units);

    Ok((65..=90)
        .map(|c: u8| c as char)
        .map(|c| {
            let filtered: Vec<char> = chars
                .iter()
                .cloned()
                .filter(|a| a.to_ascii_uppercase() != c)
                .collect();
            remaining(&filtered)
        })
        .map(|c| c.len())
        .min()
        .unwrap() as u32)
}
