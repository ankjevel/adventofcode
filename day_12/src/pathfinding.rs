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

pub fn find_path(map: &Map, start: Point, goal: Point) -> Vec<Point> {
    let can_move = |point: &Point, came_from: &Point| {
        let last_tile = map.get(came_from).unwrap();
        match map.get(&point) {
            Some(tile) => *tile as i64 - *last_tile as i64 <= 1,
            None => false,
        }
    };

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
            if visited.contains_key(&edge) || !can_move(&edge, &point) {
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
