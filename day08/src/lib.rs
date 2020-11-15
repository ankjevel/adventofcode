pub mod part_01;

pub fn parse_input(string: &str) -> Vec<u8> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .next()
        .unwrap()
        .split(' ')
        .map(|part| part.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}
