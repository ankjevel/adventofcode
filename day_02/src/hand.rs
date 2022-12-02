#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn sum(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

use Hand::*;
use Outcome::*;

impl Hand {
    pub fn new(input: &char) -> Self {
        match input {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            _ => Scissors,
        }
    }

    fn sum(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome(&self, opponent: &Hand) -> u32 {
        match opponent {
            Rock => match self {
                Rock => Draw.sum(),
                Paper => Win.sum(),
                Scissors => Lose.sum(),
            },
            Paper => match self {
                Rock => Lose.sum(),
                Paper => Draw.sum(),
                Scissors => Win.sum(),
            },
            Scissors => match self {
                Rock => Win.sum(),
                Paper => Lose.sum(),
                Scissors => Draw.sum(),
            },
        }
    }

    pub fn cheat(&self, opponent: &Hand) -> u32 {
        match self {
            Rock => {
                Lose.sum()
                    + match opponent {
                        Rock => Scissors.sum(),
                        Paper => Rock.sum(),
                        Scissors => Paper.sum(),
                    }
            }
            Paper => {
                Draw.sum()
                    + match opponent {
                        Rock => Rock.sum(),
                        Paper => Paper.sum(),
                        Scissors => Scissors.sum(),
                    }
            }
            Scissors => {
                Win.sum()
                    + match opponent {
                        Rock => Paper.sum(),
                        Paper => Scissors.sum(),
                        Scissors => Rock.sum(),
                    }
            }
        }
    }

    pub fn play(&self, opponent: &Hand) -> u32 {
        let shape_sum = self.sum();
        let outcome = self.outcome(opponent);
        shape_sum + outcome
    }
}
