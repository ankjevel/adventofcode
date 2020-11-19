use std::collections::HashMap;

use crate::{min_max, point::Point, point::Position, Input, Location};

#[derive(Debug, Clone, Default)]
pub struct Board {
    pub _board: HashMap<Location, Option<Point>>,
    pub points: Vec<Point>,
}

impl Board {
    pub fn new(input: &Vec<Input>) -> Self {
        let points = input
            .clone()
            .iter_mut()
            .map(|&mut point| Point::new(point))
            .collect();
        Self {
            points,
            ..Board::default()
        }
    }

    pub fn move_points(&self) -> Vec<Point> {
        self.points
            .clone()
            .iter_mut()
            .map(|mut point| {
                point.position.x += point.velocity.x;
                point.position.y += point.velocity.y;
                return point;
            })
            .map(|point| point.to_owned())
            .collect()
    }

    pub fn draw(&self) {
        let ((min_x, min_y), (max_x, max_y)) = min_max(&self.get_positions(&self.points));
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!(
                    "{}",
                    if self.points.contains(&Point::new_using_position(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                );
            }
            print!("\n");
        }
    }

    pub fn run(&mut self) -> u32 {
        let mut seconds = 0;

        loop {
            let new_points = self.move_points();
            let new_size = self.size(&new_points);
            let old_size = self.size(&self.points);

            if new_size <= old_size {
                seconds += 1;
                let _ = std::mem::replace(&mut self.points, new_points);
                continue;
            }

            self.draw();
            break;
        }

        seconds
    }

    fn size(&self, points: &Vec<Point>) -> isize {
        let ((min_x, min_y), (max_x, max_y)) = min_max(&self.get_positions(points));

        (max_x - min_x) * (max_y - min_y)
    }

    fn get_positions(&self, points: &Vec<Point>) -> Vec<Position> {
        points
            .clone()
            .iter()
            .map(|point| point.to_owned().position)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture.txt");

    #[test]
    fn it_creates_the_board() {
        Board::new(&parse_input(&EXAMPLE_DATA));
    }

    #[test]
    fn it_can_draw_the_example_result() {
        Board::new(&parse_input(&EXAMPLE_DATA)).run();
    }

    #[test]
    fn it_gets_the_seconds_it_takes_to_complete_correct() {
        assert_eq!(Board::new(&parse_input(&EXAMPLE_DATA)).run(), 3);
    }
}
