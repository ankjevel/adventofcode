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

#[allow(dead_code)]
fn min_max(input: &Vec<Vec<Point>>) -> (i32, i32, i32, i32) {
    let (max_x, max_y) = input.iter().fold((0, 0), |(max_x, max_y), point| {
        point.iter().fold((0, 0), |(max_x_x, max_y_y), point| {
            (
                max(max_x, max(max_x_x, point.x)),
                max(max_y, max(max_y_y, point.y)),
            )
        })
    });

    let (min_x, min_y) = input.iter().fold((0, 0), |(min_x, min_y), point| {
        point.iter().fold((0, 0), |(min_x_x, min_y_y), point| {
            (
                min(min_x, min(min_x_x, point.x)),
                min(min_y, min(min_y_y, point.y)),
            )
        })
    });

    (min_x, min_y, max_x, max_y)
}

pub fn main(input: &Vec<Vec<Point>>) -> Result<u64> {
    let mut grid = HashMap::new();
    let mut intersecting = Vec::new();
    let start_point = Point { x: 0, y: 0 };

    grid.insert(start_point, Start);

    // # for printing the board, slow process
    // let (min_x, min_y, max_x, max_y) = min_max(&input);
    // for x in min_x..max_x {
    //     for y in min_y..max_y {
    //         let state = if x == 0 && y == 0 { Start } else { Empty };
    //         let point = Point { x, y };
    //         grid.insert(point, state);
    //     }
    // }

    let mut current_wire = 0;
    for points in input.iter() {
        let mut prev = Point::new();

        for current in points.iter().peekable() {
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
