use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::Result;
use std::u64::MAX;

use super::Point;

enum State {
    Occupied(u8, u64),
    Intersect,
    Start,
    Empty,
}

use State::{Empty, Intersect, Occupied, Start};

fn calc_steps(from: i32, to: i32) -> i64 {
    (to as i64 - from as i64).abs()
}

pub fn main(input: &Vec<Vec<Point>>) -> Result<u64> {
    let mut grid = HashMap::new();
    let mut shortest_route = MAX;

    grid.insert(Point::new(), Start);

    let mut current_wire = 0;
    for wire in input.iter() {
        let mut prev = Point::new();
        let mut steps_total = 0;

        for current in wire.iter() {
            let x_neg = prev.x > current.x;
            let y_neg = prev.y > current.y;

            let mut x = prev.x;
            let mut y = prev.y;

            let mut steps_x = 0;
            let mut steps_y = 0;

            let mut check_current_state = |point: Point, current_steps: u64| {
                let state = grid.entry(point).or_insert(Empty);
                match state {
                    Empty => *state = Occupied(current_wire, current_steps),
                    Occupied(occupied_by, occupied_by_steps) => {
                        if *occupied_by != current_wire {
                            shortest_route =
                                min(shortest_route, *occupied_by_steps + current_steps);
                            *state = Intersect;
                        }
                    }
                    _ => {}
                };
            };

            let check_current_steps =
                |steps_x: i64, steps_y: i64| -> u64 { (steps_total + (steps_x + steps_y)) as u64 };

            for _ in min(prev.x, current.x)..max(prev.x, current.x) {
                x = if x_neg { x - 1 } else { x + 1 };
                steps_x = calc_steps(prev.x, x);
                check_current_state(Point { x, y }, check_current_steps(steps_x, steps_y));
            }

            for _ in min(prev.y, current.y)..max(prev.y, current.y) {
                y = if y_neg { y - 1 } else { y + 1 };
                steps_y = calc_steps(prev.y, y);
                check_current_state(Point { x, y }, check_current_steps(steps_x, steps_y));
            }

            steps_total += steps_x + steps_y;
            prev = *current;
        }

        current_wire += 1;
    }

    Ok(shortest_route)
}
