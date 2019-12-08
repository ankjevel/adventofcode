pub fn parse_input(string: &str) -> Vec<Vec<i32>> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            string
                .split(',')
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

pub mod program;
