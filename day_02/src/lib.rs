pub mod part_01;
pub mod part_02;

pub type Input = Vec<(u16, u16, char, String)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|part| {
            let mut iter = part.split(" ").map(str::trim);
            let mut range = iter.next().unwrap().split("-");

            let min = range.next().unwrap().parse::<u16>().unwrap();
            let max = range.next().unwrap().parse::<u16>().unwrap();

            let character = iter.next().unwrap().to_string().chars().next().unwrap();
            let password = iter.next().unwrap().trim().to_owned();

            (min, max, character, password)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        2-6 c: fcpwjqhcgtffzlbj
        6-9 x: xxxtwlxxx
    ";

    #[test]
    fn it_can_parse_input_as_expected() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                (2, 6, 'c', "fcpwjqhcgtffzlbj".to_string()),
                (6, 9, 'x', "xxxtwlxxx".to_string())
            ]
        );
    }
}
