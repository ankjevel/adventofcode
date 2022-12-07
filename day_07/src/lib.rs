use std::collections::{HashMap, VecDeque};

pub mod part_01;
pub mod part_02;

pub type Input = Vec<String>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect()
}

pub fn get_dirs(input: &Input) -> HashMap<String, u32> {
    let mut path: VecDeque<String> = VecDeque::new();
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for row in input.into_iter() {
        if row.starts_with('$') {
            if row.contains("$ cd") {
                let next_dir = row.replace("$ cd ", "");
                if next_dir.contains("..") {
                    path.pop_back();
                } else {
                    path.push_back(if next_dir.contains('/') {
                        next_dir
                    } else {
                        format!("{}/", next_dir)
                    });
                }
            }
        } else if !row.starts_with("dir ") {
            let n = row.split(' ').collect::<Vec<&str>>();
            let size = n[0].parse::<u32>().unwrap();
            let mut current = vec![];
            for path in path.clone().into_iter() {
                current.push(path);
                *dirs.entry(current.join("")).or_insert(0) += size;
            }
        }
    }

    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        example
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec!["example"]);
    }
}
