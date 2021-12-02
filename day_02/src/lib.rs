pub mod part_01;
pub mod part_02;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub type Input = Vec<Command>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .map(|string| {
            let split = string.split_whitespace().collect::<Vec<&str>>();
            let units = i32::from_str_radix(split[1], 10).unwrap_or(0);
            match split[0] {
                "forward" => Command::Forward(units),
                "down" => Command::Down(units),
                "up" => Command::Up(units),
                _ => panic!("unknown command"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
    forward 5
    down 5
    up 2
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![Command::Forward(5), Command::Down(5), Command::Up(2)]
        );
    }
}
