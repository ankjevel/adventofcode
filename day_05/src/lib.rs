pub fn parse_input(string: &str) -> Vec<Vec<i64>> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            string
                .split(',')
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

pub mod program;
