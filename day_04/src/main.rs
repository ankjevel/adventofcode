use std::io::Result;

mod part_01;
mod part_02;

use part_01::main as part_01;
use part_02::main as part_02;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../../input/day_04"));

    println!("part_01: {}", part_01(&input).unwrap());
    println!("part_02: {}", part_02(&input).unwrap());

    Ok(())
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .next()
        .unwrap()
        .split('-')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::part_01::meet_criteria as part_01;
    use super::part_02::meet_criteria as part_02;

    #[test]
    fn it_gets_the_right_answer_for_examples() {
        assert_eq!(part_01("111111"), true);
        assert_eq!(part_01("223450"), false);
        assert_eq!(part_01("123789"), false);
        assert_eq!(part_01("122345"), true);
        assert_eq!(part_01("135679"), false);
        assert_eq!(part_01("111123"), true);
    }

    #[test]
    fn it_gets_the_right_answer_for_examples_on_part_2() {
        assert_eq!(part_02("112233"), true);
        assert_eq!(part_02("123444"), false);
        assert_eq!(part_02("111122"), true);
    }
}
