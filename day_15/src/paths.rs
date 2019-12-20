use crate::enums::Tile;
use crate::Point;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.0 as f64 - p1.0 as f64).powf(2f64) + (p2.1 as f64 - p1.1 as f64).powf(2f64)).sqrt()
}

#[allow(dead_code)]
fn is_between(a: &Point, c: &Point, b: &Point) -> bool {
    approx_eq!(
        f64,
        distance(&a, &c) + distance(&c, &b),
        distance(&a, &b),
        ulps = 2
    )
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
struct State {
    cost: isize,
    point: Point,
}

type Map = HashMap<Point, Tile>;

pub fn best_match(input: &Map, position: &Point, visited: &Vec<Point>) -> Option<Vec<Point>> {
    let within_range = || -> Option<Vec<Point>> {
        let get_closest = |range: usize| -> Option<Vec<Point>> {
            let input = input
                .clone()
                .iter()
                .filter(|(pos, tile)| {
                    (tile == &&Tile::Unknown || tile == &&Tile::Visited) && !visited.contains(pos)
                })
                .filter(|(pos, _title)| {
                    let x = ((position.0 as f64) - (pos.0 as f64)).abs() as usize;
                    let y = ((position.1 as f64) - (pos.1 as f64)).abs() as usize;

                    x <= range && y <= range
                })
                .map(|(pos, _tile)| pos.to_owned())
                .collect::<Vec<Point>>();

            if input.len() <= 0 {
                None
            } else {
                Some(input.to_owned())
            }
        };

        let mut range = 0;
        let mut result = None;

        loop {
            if let Some(res) = get_closest(range) {
                result = Some(res);
                break;
            }
            range += 1;
            if range >= input.len() {
                break;
            }
        }

        result
    };

    let available = match within_range() {
        Some(res) => res.to_owned(),
        None => return None,
    };

    let mut steps = vec![];

    for current in available.clone() {
        let path = match find_path(&input, position.to_owned(), current.to_owned()) {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if steps.len() == 0 || path.len() < steps.len() {
            steps = path.to_owned();
        }
    }

    if steps.len() == 0 {
        None
    } else {
        Some(steps.to_owned())
    }
}

pub fn adjacent(map: &Map, point: &Point) -> Vec<Point> {
    let mut vec = Vec::new();

    for (x, y) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let new_pos = (point.0 + x, point.1 + y);
        if let Some(tile) = map.get(&new_pos) {
            if tile != &Tile::Wall {
                vec.push(new_pos.to_owned());
            }
        };
    }

    vec
}

pub fn find_leafs(map: &Map, current: &Vec<Point>) -> Vec<Point> {
    let mut new_leafs: HashSet<Point> = HashSet::new();

    for point in current {
        for (x, y) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_pos = (point.0 + x, point.1 + y);
            if let Some(tile) = map.get(&new_pos) {
                if tile == &Tile::Visited {
                    new_leafs.insert(new_pos.to_owned());
                }
            }
        }
    }

    new_leafs.into_iter().collect::<Vec<Point>>()
}

pub fn find_path(map: &Map, start: Point, goal: Point) -> Option<Vec<Point>> {
    let can_move = |point: &Point| -> bool {
        match map.get(&point) {
            Some(tile) => tile != &Tile::Wall,
            None => false,
        }
    };

    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        point: start,
    });

    let mut came_from = HashMap::new();
    came_from.insert(start, None);

    while frontier.len() != 0 {
        let current = frontier.pop();
        if current.unwrap().point == goal {
            break;
        }
        for next_point in adjacent(map, &current.unwrap().point) {
            if !came_from.contains_key(&next_point) && can_move(&next_point) {
                frontier.push(State {
                    point: next_point,
                    cost: distance(&goal, &next_point) as isize,
                });
                came_from.insert(next_point, current.map(|a| a.point));
            }
        }
    }

    let mut current = goal;
    let mut path = vec![current];

    while current != start {
        if let Some(c) = came_from.get(&current) {
            if let Some(c) = *c {
                current = c;
                path.push(current);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_goes_around_the_maze() {
        let mut map = HashMap::new();

        let c = Tile::Current;
        let v = Tile::Visited;
        let w = Tile::Wall;

        let positions = vec![
            vec![w, w, w, w, w], // [(0,0), (1,0), (2,0), (3,0), (4,0)]
            vec![w, v, v, v, w], // [(0,1), (1,1), (2,1), (3,1), (4,1)]
            vec![w, v, w, c, w], // [(0,2), (1,2), (2,2), (3,2), (4,2)]
            vec![w, v, w, w, w], // [(0,3), (1,3), (2,3), (3,3), (4,3)]
            vec![w, v, v, v, w], // [(0,4), (1,4), (2,4), (3,4), (4,4)]
            vec![w, w, w, w, w], // [(0,5), (1,5), (2,5), (3,5), (4,5)]
        ];

        for (y, row) in positions.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let (x, y) = (x.to_owned() as i64, y.to_owned() as i64);
                map.insert((x, y), tile.to_owned());
            }
        }

        assert_eq!(
            find_path(&map, (3, 2), (3, 4)),
            Some(vec![
                (3, 4),
                (2, 4),
                (1, 4),
                (1, 3),
                (1, 2),
                (1, 1),
                (2, 1),
                (3, 1),
                (3, 2)
            ])
        );
    }
}
