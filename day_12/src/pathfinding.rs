use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

use crate::{point::Point, Map};

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    point: Point,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x as f64 - p1.x as f64).powf(2.0) + (p2.y as f64 - p1.y as f64).powf(2.0)).sqrt()
}

pub fn adjacent(map: &Map, point: &Point) -> Vec<Point> {
    let mut tiles = Vec::new();

    let mut vec: Vec<(isize, isize)> = vec![(0, 1), (1, 0)];

    if point.x > 0 {
        vec.push((-1, 0));
    }

    if point.y > 0 {
        vec.push((0, -1));
    }

    for (x, y) in vec {
        let new_pos = Point {
            x: (point.x as isize + x) as usize,
            y: (point.y as isize + y) as usize,
        };
        if let Some(_) = map.get(&new_pos) {
            tiles.push(new_pos.to_owned());
        };
    }

    tiles
}

fn to_num(input: char) -> i32 {
    let n = input as i32 - 65;
    if n >= 32 {
        n - 32
    } else {
        n + 26
    }
}

pub fn find_path(map: &Map, start: Point, goal: Point) -> Vec<Point> {
    let can_move = |point: &Point, came_from: &Point| {
        let last_tile = map.get(came_from).unwrap();
        match map.get(&point) {
            Some(tile) => {
                let diff = to_num(*tile) - to_num(*last_tile);
                (last_tile == &'S' && tile == &'a')
                    || (last_tile == &'z' && tile == &'E')
                    || tile == last_tile
                    || diff <= 1
            }
            None => false,
        }
    };

    let mut frontier = BinaryHeap::new();
    // let mut distances = HashMap::new();
    let mut visited = HashMap::new();

    frontier.push(State {
        cost: 0,
        point: start,
    });

    visited.insert(start, None);
    // distances.insert(start.to_owned(), 0);

    while let Some(State { point, cost: _ }) = frontier.pop() {
        if point == goal {
            break;
        }

        // let to_point = distances.entry(point.to_owned()).or_insert(usize::MAX);
        // if cost > *to_point {
        //     continue;
        // }

        for edge in adjacent(map, &point) {
            if visited.contains_key(&edge) || !can_move(&edge, &point) {
                continue;
            }

            // let next_cost = distance(&goal, &edge) as usize;

            // let prev_cost = distances.entry(edge.to_owned()).or_insert(usize::MAX);

            // if next_cost > *prev_cost {
            //     continue;
            // }

            // frontier.push(State {
            //     point: edge,
            //     cost: next_cost.to_owned(),
            // });

            visited.insert(edge, Some(point));
            // *prev_cost = next_cost;
        }
    }

    let mut current = goal;
    let mut path = vec![current];

    while current != start {
        match visited.get(&current) {
            Some(previous) => {
                current = previous.unwrap();
                if current != start {
                    path.push(current);
                }
            }
            _ => return vec![],
        }
    }

    path
}
