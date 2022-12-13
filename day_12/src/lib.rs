use std::collections::HashMap;

use point::Point;

pub mod part_01;
pub mod part_02;
pub mod pathfinding;
pub mod point;
pub mod print;

pub type Map = HashMap<Point, u32>;

#[derive(Eq, PartialEq, Debug)]
pub struct Grid {
    start: Point,
    end: Point,
    map: Map,
}

pub type Input = Grid;

pub fn parse_input(input: &str) -> Input {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let mut start = Point::new();
    let mut end = Point::new();
    let mut map: Map = HashMap::new();
    let a = 'a' as u32;
    let z = 'z' as u32;
    for (y, row) in lines.into_iter().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            let point = Point { x, y };
            map.insert(
                point,
                match tile {
                    'S' => {
                        start = point.to_owned();
                        0
                    }
                    'E' => {
                        end = point.to_owned();
                        z - a + 2
                    }
                    _ => (tile as u32) - a + 1,
                },
            );
        }
    }
    Grid { start, end, map }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        SE
        ab
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            Grid {
                start: Point { x: 0, y: 0 },
                end: Point { x: 1, y: 0 },
                map: HashMap::from_iter(
                    vec![
                        (Point { x: 0, y: 0 }, 0),
                        (Point { x: 0, y: 1 }, 1),
                        (Point { x: 1, y: 1 }, 2),
                        (Point { x: 1, y: 0 }, 27),
                    ]
                    .into_iter()
                )
            }
        );
    }
}
