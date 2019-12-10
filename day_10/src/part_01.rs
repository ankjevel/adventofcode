use crate::point::Point;

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x as f64 - p1.x as f64).powf(2f64) + (p2.y as f64 - p1.y as f64).powf(2f64)).sqrt()
}

fn is_between(a: &Point, c: &Point, b: &Point) -> bool {
    approx_eq!(
        f64,
        distance(&a, &c) + distance(&c, &b),
        distance(&a, &b),
        ulps = 2
    )
}

pub fn main(input: &Vec<Point>) -> (u32, Option<Point>) {
    let mut max = 0;
    let mut best_match = None;

    'a: for a in input {
        let mut new_max = 0;
        let mut matches = Vec::new();

        'b: for b in input {
            if b == a {
                continue 'b;
            }

            let mut intersected = false;

            'c: for c in input {
                if a == c || b == c {
                    continue 'c;
                }

                if is_between(a, c, b) {
                    intersected = true;
                    break 'c;
                }
            }

            if !intersected {
                matches.push(b.clone());
                new_max += 1;
            }
        }

        if new_max > max {
            best_match = Some(a.clone());
            max = new_max;
        }
    }

    (max, best_match)
}
