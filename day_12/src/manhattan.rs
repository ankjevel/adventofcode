use crate::point::Point;

pub fn manhattan(point: Point, current: Point) -> u64 {
    ((point.x as i64 - current.x as i64).abs() + (point.y as i64 - current.y as i64).abs()) as u64
}
