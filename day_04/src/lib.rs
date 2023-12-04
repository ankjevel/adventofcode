pub mod part_01;

pub type Input = Vec<(Vec<u16>, Vec<u16>)>;

fn numbers(input: &str) -> Vec<u16> {
    input
        .split(' ')
        .filter(|string| !string.trim().is_empty())
        .map(|string| string.trim().parse::<_>().unwrap())
        .collect()
}

pub fn parse_input(input: &str) -> Input {
    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|string| {
            let card = string.split(": ").collect::<Vec<_>>()[1];
            let split = card.split(" | ").collect::<Vec<_>>();
            (numbers(split[0]), numbers(split[1]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                (
                    vec![41, 48, 83, 86, 17], //
                    vec![83, 86, 6, 31, 17, 9, 48, 53]
                ),
                (
                    vec![13, 32, 20, 16, 61], //
                    vec![61, 30, 68, 82, 17, 32, 24, 19]
                )
            ]
        );
    }
}
