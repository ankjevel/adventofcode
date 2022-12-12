use std::collections::HashMap;

use point::Point;

pub mod part_01;
pub mod part_02;
pub mod pathfinding;
pub mod point;

type Map = HashMap<Point, char>;

#[derive(Eq, PartialEq, Debug)]
pub struct Grid {
    start: Point,
    end: Point,
    map: Map,
}

pub type Input = Grid;

pub fn parse_input(input: &str) -> Input {
    let mut lines = input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    lines.reverse();

    let mut start = Point::new();
    let mut end = Point::new();
    let mut map: Map = HashMap::new();
    for (y, row) in lines.into_iter().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            let point = Point { x, y };
            match tile {
                'S' => {
                    start = point.to_owned();
                }
                'E' => {
                    end = point.to_owned();
                }
                _ => {}
            }
            map.insert(point, tile);
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
                start: Point { x: 0, y: 1 },
                end: Point { x: 1, y: 1 },
                map: HashMap::from_iter(
                    vec![
                        (Point { x: 0, y: 1 }, 'S'),
                        (Point { x: 1, y: 1 }, 'E'),
                        (Point { x: 0, y: 0 }, 'a'),
                        (Point { x: 1, y: 0 }, 'b'),
                    ]
                    .into_iter()
                )
            }
        );
    }
}
