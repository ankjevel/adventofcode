use crate::{parse_input, Steps};
use std::{io::Result, str};

fn letter_to_duration(letter: char, default_duration: u8) -> u8 {
    letter as u8 - 64 + default_duration
}

pub type Running = (char, u8);

pub fn main(input: &str, number_of_workers: usize, default_duration: u8) -> Result<usize> {
    let modify_active_workers = move |active_workers: &mut Vec<Running>, steps: &mut Steps| {
        active_workers.iter_mut().for_each(|(letter, steps_left)| {
            *steps_left -= 1;
            if *steps_left > 0 {
                return;
            }
            steps.remove(&letter);
            for (_, required) in steps.iter_mut() {
                if !required.contains(&letter) {
                    continue;
                }
                required.remove(&letter);
            }
        });
        active_workers.retain(|(_, steps_left)| steps_left != &0);
    };

    let get_available_letters = |active_workers: &Vec<Running>, steps: &Steps| -> Vec<char> {
        let mut available: Vec<char> = steps
            .iter()
            .filter(|(letter, required)| {
                required.is_empty()
                    && active_workers.len() < number_of_workers
                    && active_workers
                        .iter()
                        .find(|(working_on, _)| &working_on == letter)
                        .is_none()
            })
            .map(|(step, _)| *step)
            .collect();
        available.sort();
        return available;
    };

    let mut steps = parse_input(input.clone());
    let mut active_workers: Vec<Running> = vec![];
    let mut duration: usize = 0;

    while !steps.is_empty() {
        duration += 1;
        modify_active_workers(&mut active_workers, &mut steps);

        if active_workers.len() == number_of_workers {
            continue;
        }

        get_available_letters(&active_workers, &steps)
            .iter()
            .for_each(|letter| {
                if active_workers.len() == number_of_workers {
                    return;
                }
                active_workers.push((*letter, letter_to_duration(*letter, default_duration)));
            })
    }
    Ok(if duration > 0 { duration - 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture.txt");

    #[test]
    fn it_should_return_the_expected_duration_for_letter() {
        let example_duration = 0;
        assert_eq!(letter_to_duration('A', example_duration), 1);
        assert_eq!(letter_to_duration('Z', example_duration), 26);

        let real_duration = 60;
        assert_eq!(letter_to_duration('A', real_duration), 61);
        assert_eq!(letter_to_duration('Z', real_duration), 86);
    }

    #[test]
    fn it_should_register_the_same_duration_as_part_two() {
        let example_duration = 0;
        let number_of_workers = 2;

        assert_eq!(
            main(EXAMPLE_DATA, number_of_workers, example_duration).unwrap(),
            15
        );
    }
}
