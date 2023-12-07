use std::cmp::Ordering;

pub mod hand;
pub mod part_01;
pub mod part_02;

pub type Hand = Vec<char>;
type Input = Vec<(Hand, u128)>;

pub fn parse_input(input: &str) -> Input {
    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|line| {
            let line_split = line.split(' ').collect::<Vec<_>>();
            let cards = line_split[0].chars().collect();
            let bid = line_split[1].parse().unwrap();
            (cards, bid)
        })
        .collect()
}

pub fn sum(input: &Input, sort: Box<dyn Fn(&Hand, &Hand) -> Ordering>) -> u128 {
    let mut input = input.to_owned();
    input.sort_by(|(a_cards, _), (b_cards, _)| sort(&a_cards, &b_cards));
    let iter = input.into_iter().enumerate();
    iter.map(|(i, (_, bid))| bid * ((i as u128) + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
    32T3K 765
    T55J5 684
    ";

    #[test]
    fn it_parses_input() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                (vec!['3', '2', 'T', '3', 'K'], 765),
                (vec!['T', '5', '5', 'J', '5'], 684)
            ]
        );
    }
}
