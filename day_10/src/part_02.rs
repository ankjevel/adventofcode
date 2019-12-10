use crate::{angle::Angle, part_01::is_between, point::Point};

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::BTreeMap,
    f64::consts::PI,
};

type Map = BTreeMap<Angle, Vec<Point>>;

fn to_map(input: &Vec<Point>, start: &Point) -> Map {
    let mut map: Map = BTreeMap::new();

    for point in input {
        if point == start {
            continue;
        }
        let ax = point.x as f64;
        let ay = point.y as f64;
        let bx = start.x as f64;
        let by = start.y as f64;
        let mut angle = (ay - by).atan2(ax - bx) * (180f64 / PI) + 90f64;

        if angle < 0f64 {
            angle = 360f64 + angle;
        }

        let reference = map.entry(Angle::new(&angle)).or_insert(Vec::new());
        reference.push(point.clone());
        reference.sort_by(|a, b| {
            if is_between(start, a, b) {
                Greater
            } else if is_between(start, b, a) {
                Less
            } else {
                Equal
            }
        });
    }

    map
}

pub fn main(input: &Vec<Point>, maybe_start: &Option<Point>) -> u64 {
    let start = maybe_start.unwrap_or_else(|| panic!("missing start position"));
    let mut map = to_map(&input, &start);
    let mut vec = Vec::new();

    for (_, item) in &map {
        vec.push(item.clone());
    }

    map.clear(); // freeeeeee!
    let mut ordered = Vec::new();
    'outer: loop {
        'inner: for x in vec.iter_mut() {
            if ordered.len() >= 200 {
                break 'outer;
            }

            if x.len() == 0 {
                continue 'inner;
            }

            let this = x.into_iter().next().unwrap();
            ordered.push(*this);
        }
        break 'outer;
    }
    let last = ordered.into_iter().last().unwrap();
    (last.x * 100) + last.y
}
