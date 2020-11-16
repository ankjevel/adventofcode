#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

use regex::{Captures, Regex};
use std::collections::VecDeque;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\d+) players; last marble is worth (\d+) points(?:: high score is (\d+))?")
            .unwrap();
}

#[derive(Default)]
pub struct Game {
    players: u128,
    run_until: u128,
    iterations: usize,
    scores: Vec<u128>,
    circle: VecDeque<u128>,
}

impl Game {
    pub fn new(players: u128, run_until: u128) -> Self {
        Self {
            players,
            run_until,
            scores: vec![0; players as usize],
            ..Game::default()
        }
    }

    fn move_current_forwards(&mut self) {
        for _ in 0..2 {
            if let Some(value) = self.circle.pop_front() {
                self.circle.push_back(value)
            }
        }
    }

    fn move_current_backwards(&mut self) {
        for _ in 0..7 {
            if let Some(value) = self.circle.pop_back() {
                self.circle.push_front(value)
            }
        }
    }

    fn iterate(&mut self) {
        if self.is_special_case() {
            self.move_current_backwards();
            let score = self.circle.pop_front().unwrap();
            self.store_score(score);
        } else {
            self.move_current_forwards();
            self.circle.push_front(self.iterations as u128);
        }
        self.set_next();
    }

    fn get_high_score(&mut self) -> u128 {
        *self.scores.iter().max().unwrap()
    }

    fn store_score(&mut self, score: u128) {
        let index = self.iterations % self.players as usize;
        if let Some(player) = self.scores.get_mut(index) {
            *player += score + self.iterations as u128;
        };
    }

    fn is_special_case(&mut self) -> bool {
        self.iterations > 2 && self.iterations % 23 == 0
    }

    fn set_next(&mut self) {
        self.iterations += 1;
    }

    fn should_break(&mut self) -> bool {
        self.iterations > self.run_until as usize
    }

    pub fn run(&mut self) -> u128 {
        loop {
            self.iterate();
            if self.should_break() {
                break;
            }
        }

        self.get_high_score()
    }
}

pub fn parse_input(string: &str) -> Vec<(u128, u128, Option<u128>)> {
    fn to_number(input: &Captures, index: usize) -> u128 {
        if let Some(string) = input.get(index) {
            string.as_str().parse::<_>().unwrap()
        } else {
            0
        }
    }

    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            if !RE.is_match(&string) {
                panic!("none matching for {}", string);
            }
            let cap = RE.captures(&string).unwrap();
            (
                to_number(&cap, 1),
                to_number(&cap, 2),
                match to_number(&cap, 3) {
                    0 => None,
                    i => Some(i),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture.txt");
    const PUZZLE_INPUT: &'static str = include_str!("../../input/day_09");

    #[test]
    fn it_parses_the_example_data() {
        parse_input(&EXAMPLE_DATA);
    }

    #[test]
    fn it_parses_the_puzzle_input() {
        parse_input(&PUZZLE_INPUT);
    }

    #[test]
    fn it_gives_the_results_as_example() {
        assert_eq!(Game::new(9, 25).run(), 32);
    }

    #[test]
    fn it_gives_the_same_results_as_extra_examples() {
        let inputs = parse_input(&EXAMPLE_DATA);
        for (players, run_until, high_score) in inputs {
            assert_eq!(Game::new(players, run_until).run(), high_score.unwrap());
        }
    }
}
