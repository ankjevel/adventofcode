use point::Point;
use std::{cmp, io};

fn manhattan(point: Point, current: Point) -> u32 {
    ((point.x - current.x).abs() + (point.y - current.y).abs()) as u32
}

pub fn main(points: &Vec<Point>, limit: u32) -> io::Result<u32> {
    let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), point| {
        (cmp::max(max_x, point.x + 2), cmp::max(max_y, point.y + 2))
    });

    let mut size = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            let current = Point { x, y };
            let total = points.iter().fold(0, |mut sum, &point| {
                sum += manhattan(point, current);

                sum
            });

            if total < limit {
                size += 1;
            }
        }
    }

    Ok(size)
}
