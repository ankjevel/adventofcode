use point::Point;
use std::collections::{HashMap, HashSet};
use std::{cmp, io};

enum State {
    Claim(Point),
    Owned(Point),
    Neutral,
    Empty,
}

pub fn main(points: &Vec<Point>) -> io::Result<u32> {
    let mut grid = HashMap::new();
    let mut infinity = HashSet::new();
    let mut count = HashMap::new();

    let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), point| {
        (cmp::max(max_x, point.x + 2), cmp::max(max_y, point.y + 2))
    });

    for x in 0..max_x {
        for y in 0..max_y {
            grid.insert(Point { x, y }, State::Empty);
        }
    }

    let mut boundaries: Vec<(Point, HashSet<Point>)> = points
        .iter()
        .map(|&point| {
            grid.insert(point, State::Owned(point));
            count.insert(point, 1);

            let mut boundary = HashSet::new();
            for p in new_boundaries(&point) {
                boundary.insert(p);
            }

            (point, boundary)
        })
        .collect();

    'main_loop: loop {
        boundaries = boundaries
            .into_iter()
            .map(|(point, boundary)| {
                let mut new_boundary = HashSet::new();

                for b in boundary {
                    let mut state = grid.entry(b).or_insert(State::Empty);

                    match state {
                        State::Empty => *state = State::Claim(point),
                        State::Claim(_) => {
                            *state = State::Neutral;
                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }

                    if b.x <= 0 || b.x >= max_x || b.y <= 0 || b.y >= max_y {
                        infinity.insert(point);
                        continue;
                    }

                    for p in new_boundaries(&b) {
                        new_boundary.insert(p);
                    }
                }

                (point, new_boundary)
            })
            .collect();

        for (_, state) in grid.iter_mut() {
            match *state {
                State::Claim(point) => {
                    *state = State::Owned(point);
                    count.entry(point).and_modify(|c| *c += 1);
                }
                _ => {},
            }
        }

        if boundaries.iter().all(|(_, b)| b.is_empty()) {
            break 'main_loop;
        }
    }

    Ok(count
        .iter()
        .filter(|(p, _)| !infinity.contains(*p))
        .map(|(_, c)| *c)
        .max()
        .unwrap())
}

fn new_boundaries(point: &Point) -> Vec<Point> {
    vec![
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
    ]
}
