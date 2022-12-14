use std::collections::BTreeMap;

use crate::{point::Point, Map};

fn join<'a>(a: String, b: String) -> String {
    let (a, b) = (a.to_owned(), b.to_owned());
    let c = [a, b].concat();

    c.to_string()
}

fn max(map: &Vec<Point>) -> (usize, usize) {
    let (x, y) = map.iter().fold((0, 0), |mut acc, point| {
        let (x, y) = (point.x, point.y);
        if acc.0 < x {
            acc.0 = x.to_owned();
        }
        if acc.1 < y {
            acc.1 = y.to_owned();
        }
        acc
    });

    (x, y)
}

fn grid_as_tree_map(
    grid: &Map,
    max_x: &usize,
    max_y: &usize,
    path: &Vec<Point>,
) -> BTreeMap<Point, String> {
    let mut tree_map = BTreeMap::new();
    for y in 0..=(max_y + 1) {
        for x in 0..=(max_x + 1) {
            let point = Point { x, y };
            let key = grid.get(&point).unwrap_or(&(' ' as u32)).to_owned() + 'a' as u32 - 1;

            let key = if key < 'a' as u32 {
                "S".to_string()
            } else if key == 123 {
                "E".to_string()
            } else {
                char::from_u32(key).unwrap_or(' ').to_string()
            };

            let key = if path.contains(&point) {
                format!("\x1b[93m{}\x1b[0m", key)
            } else if key == "E" {
                " ".to_string()
            } else {
                key
            };

            tree_map.insert(point, key);
        }
    }
    tree_map
}

pub fn print(path: &Vec<Point>, map: &Map) {
    let (max_x, max_y) = max(&path);

    let mut current = 0;
    let mut string = "".to_string();
    let mut out = Vec::new();
    for (point, tile) in grid_as_tree_map(&map, &max_x, &max_y, &path) {
        if point.y != current {
            out.push(string.to_owned());
            string = "".to_string();
            current = point.y.to_owned();
        }

        string = join(string, tile.to_string())
    }

    println!("{}", out.join("\r\n"));
}
