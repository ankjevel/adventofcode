use std::io;

pub fn main(units: &Vec<char>) -> io::Result<u32> {
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

    Ok(remaining.len() as u32)
}
