pub mod part_01;
pub mod part_02;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    pub fn new() -> Self {
        Self::init(0, 0, 0)
    }

    pub fn init(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

pub type Input = Vec<Vec<Bag>>;

pub fn parse_input(input: &str) -> Input {
    input
        .trim_start()
        .trim_end()
        .lines()
        .map(str::trim)
        .map(|string| {
            string.split(": ").collect::<Vec<&str>>()[1]
                .split("; ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|record| {
                    let games = record.split(", ").collect::<Vec<&str>>();
                    let mut bag = Bag::new();

                    games.iter().for_each(|list| {
                        let cubes = list.split(' ').next().unwrap().parse::<u32>().unwrap();
                        if list.ends_with("red") {
                            bag.red = cubes;
                        }
                        if list.ends_with("green") {
                            bag.green = cubes;
                        }
                        if list.ends_with("blue") {
                            bag.blue = cubes;
                        }
                    });
                    bag
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                vec![Bag::init(4, 0, 3), Bag::init(1, 2, 6), Bag::init(0, 2, 0)],
                vec![Bag::init(0, 2, 1), Bag::init(1, 3, 4), Bag::init(0, 1, 1)]
            ]
        );
    }
}
