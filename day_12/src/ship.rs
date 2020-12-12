use crate::{
    action::Action::{self, *},
    direction::Direction,
    point::Point,
    Input,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Ship {
    facing: Direction,
    x: isize,
    y: isize,
    waypoint: Point,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing: Direction::Right,
            x: 0,
            y: 0,
            waypoint: Point { x: 10, y: -1 },
        }
    }
}

impl Ship {
    pub fn follow_instructions(mut self, input: &Input) -> Point {
        for (action, value) in input {
            match action {
                North | South | East | West => self.directional_move(action, value),
                Left | Right => self.turn(action, value),
                Forward => self.forward(value),
            }
        }
        self.point()
    }

    fn turn(&mut self, action: &Action, value: &u16) {
        let turns = ((*value as f32) / 90.0).floor() as u16;
        self.facing.turn(action, turns);
    }

    fn forward(&mut self, value: &u16) {
        let value = value.to_owned() as isize;
        match self.facing {
            Direction::Up => self.y -= value,
            Direction::Left => self.x -= value,
            Direction::Down => self.y += value,
            Direction::Right => self.x += value,
        }
    }

    fn directional_move(&mut self, action: &Action, value: &u16) {
        let value = value.to_owned() as isize;
        match action {
            North => self.y -= value,
            East => self.x += value,
            South => self.y += value,
            West | _ => self.x -= value,
        }
    }

    fn point(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn follow_waypoint(mut self, input: &Input) -> Point {
        for (action, value) in input {
            match action {
                North | South | East | West => self.move_waypoint(action, value),
                Left | Right => self.rotate_waypoint(action, value),
                Forward => self.move_towards_waypoint(value),
            }
        }
        self.point()
    }

    fn move_waypoint(&mut self, action: &Action, value: &u16) {
        let value = value.to_owned() as isize;
        match action {
            North => self.waypoint.y -= value,
            South => self.waypoint.y += value,
            East => self.waypoint.x += value,
            West | _ => self.waypoint.x -= value,
        };
    }

    fn rotate_waypoint(&mut self, action: &Action, value: &u16) {
        let degrees = *value;
        let clockwise = action == &Action::Right;

        let direction = if clockwise { 360 - degrees } else { degrees };
        let (x, y) = (self.waypoint.x, self.waypoint.y);

        self.waypoint = match direction {
            270 => Point { x: -y, y: x },
            180 => Point { x: -x, y: -y },
            90 => Point { x: y, y: -x },
            0 | _ => Point { x, y },
        }
    }

    fn move_towards_waypoint(&mut self, value: &u16) {
        let value = value.to_owned() as isize;
        let (x, y) = (self.waypoint.x, self.waypoint.y);
        let (nx, ny) = (x * value, y * value);
        self.x += nx;
        self.y += ny;
    }
}
