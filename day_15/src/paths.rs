use crate::enums::Tile;
use crate::Point;
use std::collections::{BinaryHeap, HashMap};

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

type Map = HashMap<Point, Tile>;

pub fn best_match(input: &Map, position: &Point, visited: &Vec<Point>) -> Option<Vec<Point>> {
    let available = input
        .clone()
        .iter()
        .filter(|(pos, tile)| {
            (tile == &&Tile::Unknown || tile == &&Tile::Visited) && !visited.contains(pos)
        })
        .map(|(pos, _tile)| pos.to_owned())
        .collect::<Vec<Point>>();

    if available.len() <= 0 {
        return None;
    }

    let mut steps = vec![];

    for current in available.clone() {
        let path = match find_path(&input, position.to_owned(), current.to_owned()) {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if path.len() < steps.len() {
            steps = path.to_owned();
        }
    }

    if steps.len() == 0 {
        None
    } else {
        Some(steps.to_owned())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
struct State {
    cost: isize,
    point: Point,
}

pub fn find_path(map: &Map, start: Point, goal: Point) -> Option<Vec<Point>> {
    let adjacent = |point: &Point| -> Vec<Point> {
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
    };

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
        for next_point in adjacent(&current.unwrap().point) {
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
