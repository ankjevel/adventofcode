mod part_01;
mod part_02;

use std::io;

fn main() -> io::Result<()> {
    let records = parse_input(include_str!("../../input/day_05"));

    println!("part_01: {}", part_01::main(&records).unwrap());

    println!("part_02: {}", part_02::main(&records).unwrap());

    Ok(())
}

fn parse_input(string: &str) -> Vec<char> {
    string.trim().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "dabAcCaCBAcCcaDA";

    #[test]
    fn how_many_units_remain_after_fully_reacting_the_polymer() {
        let result = part_01::main(&parse_input(&EXAMPLE_DATA)).unwrap();

        assert_eq!(result, 10)
    }

    #[test]
    fn length_of_the_shortest_polymer() {
        let result = part_02::main(&parse_input(&EXAMPLE_DATA)).unwrap();

        assert_eq!(result, 4)
    }
}
