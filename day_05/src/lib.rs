pub mod part_01;
pub mod part_02;

pub type Number = u8;
pub type Input = (Vec<(Number, Number)>, Vec<Vec<Number>>);

pub fn parse_input(input: &str) -> Input {
    let input = input
        .trim()
        .lines()
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let (page_ordering, page_numbers) =
        input.split_at(input.iter().position(String::is_empty).unwrap());

    let page_ordering = page_ordering
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split = line
                .split("|")
                .map(|part| part.parse().unwrap())
                .collect::<Vec<_>>();
            (split[0], split[1])
        })
        .collect();

    let page_numbers = page_numbers
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(",").map(|part| part.parse().unwrap()).collect())
        .collect();

    (page_ordering, page_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        47|29
        75|13
        53|13

        75,47,61,53,29
        53,29,13
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            (
                vec![(47, 29), (75, 13), (53, 13)],
                vec![vec![75, 47, 61, 53, 29], vec![53, 29, 13]]
            )
        );
    }
}
