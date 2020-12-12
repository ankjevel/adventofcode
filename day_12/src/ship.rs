use crate::{
    action::Action::{self, *},
    direction::Direction,
    point::Point,
    Input,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub struct Ship {
    facing: Direction,
    x: isize,
    y: isize,
}

impl Ship {
    fn turn(&mut self, action: &Action) {
        self.facing.turn(action);
    }

    fn forward(&mut self, value: &u16) {
        let value = value.to_owned() as isize;
        match self.facing {
            Direction::Up => self.y += value,
            Direction::Left => self.x -= value,
            Direction::Down => self.y -= value,
            Direction::Right => self.x += value,
        }
    }

    fn directional_move(&mut self, action: &Action, value: &u16) {
        let value = value.to_owned() as isize;
        match action {
            North => self.y += value,
            East => self.x += value,
            South => self.y -= value,
            West | _ => self.x -= value,
        }
    }

    pub fn follow_instruction(mut self, input: &Input) -> Point {
        for (action, value) in input {
            println!("action={:?}\taction={}", action, value);
            match action {
                North | South | East | West => self.directional_move(action, value),
                Left | Right => self.turn(action),
                Forward => self.forward(value),
            }
        }

        println!("x={}\ty={}", self.x, self.y);

        Point {
            x: self.x,
            y: self.y,
        }
    }
}
