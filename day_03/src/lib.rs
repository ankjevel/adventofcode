#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod part_01;
pub mod part_02;

use regex::Regex;

lazy_static! {
    static ref ROW: Regex =
        Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)")
            .unwrap();
}

pub type Number = u64;
pub type Input = Vec<Instruction>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Instruction {
    mul: Option<(Number, Number)>,
    skip: Option<bool>,
}

impl Instruction {
    pub fn mul(left: Number, right: Number) -> Self {
        Self {
            mul: Some((left, right)),
            skip: None,
        }
    }

    pub fn skip(skip: bool) -> Self {
        Self {
            mul: None,
            skip: Some(skip),
        }
    }

    pub fn sum(&self) -> Number {
        if let Some((left, right)) = self.mul {
            left * right
        } else {
            0
        }
    }
}

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .fold(vec![], |mut all, line| {
            all.extend(
                ROW.captures_iter(&line)
                    .map(|matched| {
                        if let Some(left) = matched.name("left") {
                            Instruction::mul(
                                left.as_str().parse().unwrap(),
                                matched.name("right").unwrap().as_str().parse().unwrap(),
                            )
                        } else {
                            Instruction::skip(matched.name("dont").is_some())
                        }
                    })
                    .collect::<Vec<_>>(),
            );

            all
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            vec![
                Instruction::mul(2, 4),
                Instruction::mul(5, 5),
                Instruction::mul(11, 8),
                Instruction::mul(8, 5)
            ]
        );
        assert_eq!(
            parse_input(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            vec![
                Instruction::mul(2, 4),
                Instruction::skip(true),
                Instruction::mul(5, 5),
                Instruction::mul(11, 8),
                Instruction::skip(false),
                Instruction::mul(8, 5)
            ]
        )
    }
}
