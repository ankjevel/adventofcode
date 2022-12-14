use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use crate::{point::Point, Map};

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

pub fn bfs(
    map: &Map,
    start: Point,
    goal: Point,
    can_move: impl Fn(&Point, &Point) -> bool,
) -> Vec<Point> {
    let mut frontier = VecDeque::new();
    let mut visited = HashMap::new();

    frontier.push_front(start);
    visited.insert(start, None);
    while !frontier.is_empty() {
        let point = frontier.pop_front().unwrap();
        if point == goal {
            break;
        }

        for edge in adjacent(map, &point) {
            if !can_move(&edge, &point) {
                continue;
            }
            if visited.contains_key(&edge) {
                continue;
            }
            frontier.push_back(edge);
            visited.insert(edge, Some(point));
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
