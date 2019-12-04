use std::cmp::{max, min};
use std::collections::HashMap;
use std::i32::MAX;
use std::io::Result;

use super::manhattan::manhattan;
use super::Point;

enum State {
    Occupied(u8),
    Intersect(Point),
    Start,
    Empty,
}

use State::{Empty, Intersect, Occupied, Start};

pub fn main(input: &Vec<Vec<Point>>) -> Result<u64> {
    let mut grid = HashMap::new();
    let mut intersecting = Vec::new();
    let start_point = Point { x: 0, y: 0 };

    grid.insert(start_point, Start);

    let mut current_wire = 0;
    for points in input.iter() {
        let mut prev = Point::new();

        for current in points.iter() {
            for x in min(prev.x, current.x)..=max(prev.x, current.x) {
                for y in min(prev.y, current.y)..=max(prev.y, current.y) {
                    let point = Point { x, y };
                    let state = grid.entry(point).or_insert(Empty);

                    match state {
                        Empty => *state = Occupied(current_wire),
                        Occupied(occupied_by) => {
                            if *occupied_by == current_wire {
                                continue;
                            }
                            *state = Intersect(point);
                            intersecting.push(point);
                        }
                        _ => {}
                    };
                }
            }

            prev = *current;
        }

        current_wire += 1;
    }

    let min = intersecting.iter().fold(
        manhattan(start_point, Point { x: MAX, y: MAX }),
        |sum, &point| min(sum, manhattan(start_point, point)),
    );

    Ok(min)
}
