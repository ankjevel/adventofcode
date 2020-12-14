pub mod chinese_remainder;
pub mod part_01;
pub mod part_02;

pub type Input = (u64, Vec<Option<u64>>);

pub fn parse_input(input: &str) -> Input {
    let mut input = input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .into_iter();

    let earliest = input
        .next()
        .unwrap_or("".to_string())
        .parse::<_>()
        .unwrap_or(0);

    let busses = input.next().unwrap_or("".to_string());
    let busses = busses
        .split(",")
        .into_iter()
        .map(|string| {
            if string == "x" {
                None
            } else {
                Some(string.parse::<_>().unwrap_or(0))
            }
        })
        .collect::<Vec<_>>();

    (earliest, busses)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        939
        7,13,x,x,59,x,31,19
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            (
                939,
                vec![
                    Some(7),
                    Some(13),
                    None,
                    None,
                    Some(59),
                    None,
                    Some(31),
                    Some(19)
                ]
            )
        );
    }
}
