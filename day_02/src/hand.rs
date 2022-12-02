#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

use Hand::*;

impl Hand {
    pub fn new(input: &char) -> Self {
        match input {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            _ => Scissors,
        }
    }

    fn shape_sum(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome(&self, opponent: &Hand) -> u32 {
        match opponent {
            Rock => match self {
                Rock => 3,
                Paper => 6,
                Scissors => 0,
            },
            Paper => match self {
                Rock => 0,
                Paper => 3,
                Scissors => 6,
            },
            Scissors => match self {
                Rock => 6,
                Paper => 0,
                Scissors => 3,
            },
        }
    }

    pub fn round(&self, opponent: &Hand) -> u32 {
        let shape_sum = self.shape_sum();
        let outcome = self.outcome(opponent);
        shape_sum + outcome
    }
}
