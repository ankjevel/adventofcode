use std::io;

mod part_01;
mod part_02;

fn main() -> io::Result<()> {
    let input = parse_input(include_str!("../../input/day_01"));

    let part_01_fuel = part_01::main(&input).unwrap().iter().sum::<i32>();
    let part_02_sum_of_fuel = part_02::main(&input).unwrap();

    println!("part 1: {}", part_01_fuel);
    println!("part 2: {}", part_02_sum_of_fuel);

    Ok(())
}

fn parse_input(string: &str) -> Vec<i32> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| string.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_01: &'static str = "
    12
    14
    1969
    100756";

    const EXAMPLE_DATA_02: &'static str = "
    14
    1969
    100756";

    #[test]
    fn examples_for_part_1() {
        assert_eq!(
            part_01::main(&parse_input(EXAMPLE_DATA_01)).unwrap(),
            [2, 2, 654, 33583]
        )
    }

    #[test]
    fn reduce_results_for_example_part_1() {
        assert_eq!(
            part_01::main(&parse_input(EXAMPLE_DATA_01))
                .unwrap()
                .iter()
                .sum::<i32>(),
            2 + 2 + 654 + 33583
        )
    }

    #[test]
    fn example_for_part_2() {
        assert_eq!(
            part_02::main(&parse_input(EXAMPLE_DATA_02)).unwrap(),
            2 + 966 + 50346
        )
    }
}
