pub mod part_01;
pub mod part_02;

pub type Input = Vec<Vec<String>>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .into_iter()
        .fold(vec![vec![]], |mut list, string| {
            if string.is_empty() {
                list.push(vec![]);
            } else {
                let last = list.last_mut().unwrap();
                last.push(string.to_string());
            }
            list
        })
        .into_iter()
        .filter(|list| list.len() > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        abc

        a
        b


        ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![vec!["abc"], vec!["a", "b"]]
        );
    }
}
