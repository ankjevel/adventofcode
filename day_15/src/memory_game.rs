use std::collections::HashMap;

use crate::Input;

pub fn play(input: &Input, iterations: usize) -> u64 {
    let mut spoken: HashMap<u64, (Option<usize>, Option<usize>)> = HashMap::new();

    for (index, number) in input.iter().enumerate() {
        spoken.insert(*number, (Some(index), None));
    }

    let mut last_number_spoken = input.iter().last().unwrap_or(&0).to_owned();
    for index in input.len()..iterations {
        let (last_time_spoken, previous_time_before) = spoken
            .get(&last_number_spoken)
            .unwrap_or(&(None, None))
            .to_owned();

        if last_time_spoken.is_none() || previous_time_before.is_none() {
            last_number_spoken = 0;
        } else {
            let last_time_spoken = last_time_spoken.unwrap();
            let previous_time_before = previous_time_before.unwrap();
            last_number_spoken = (last_time_spoken - previous_time_before) as u64;
        }

        match spoken.entry(last_number_spoken).or_insert((None, None)) {
            entry => {
                entry.1 = entry.0;
                entry.0 = Some(index);
            }
        };
    }

    last_number_spoken
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_examples_correct() {
        assert_eq!(play(&parse_input("1,3,2"), 2020), 1);
        assert_eq!(play(&parse_input("2,1,3"), 2020), 10);
        assert_eq!(play(&parse_input("1,2,3"), 2020), 27);
        assert_eq!(play(&parse_input("2,3,1"), 2020), 78);
        assert_eq!(play(&parse_input("3,2,1"), 2020), 438);
        assert_eq!(play(&parse_input("3,1,2"), 2020), 1836);
    }
}
