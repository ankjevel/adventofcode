use std::io::Result;

pub fn meet_criteria(input: &str) -> bool {
    let bytes: Vec<_> = input
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    let mut pairs = false;
    let mut never_decrese = true;
    for i in 1..bytes.len() {
        let current = bytes[i];
        let prev = bytes[i - 1];
        if never_decrese == false {
            break;
        }

        pairs = pairs || current == prev;

        never_decrese = current >= prev;
    }

    pairs && never_decrese
}

pub fn main(input: &str) -> Result<u32> {
    let range = &input
        .split('-')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut count = 0;
    for i in range[0]..range[1] {
        let string = i.to_string();
        if meet_criteria(&string) {
            count += 1;
        }
    }

    Ok(count)
}
