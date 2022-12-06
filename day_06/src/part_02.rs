use std::{
    collections::{HashSet, VecDeque},
    io::Result,
};

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    let mut markers: VecDeque<char> = VecDeque::new();
    let mut completed_at = 0;
    for (i, char) in input.to_owned().into_iter().enumerate() {
        markers.push_front(char.to_owned());

        if markers.len() > 14 {
            markers.pop_back();
        }

        let set: HashSet<char> = HashSet::from_iter(markers.clone().into_iter());
        if set.len() != 14 {
            continue;
        }

        completed_at = i + 1;
        break;
    }

    Ok(completed_at)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        let parse = |input: &str| -> Result<usize> { main(&parse_input(&input)) };

        assert_eq!(parse(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 19);
        assert_eq!(parse(&"bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
        assert_eq!(parse(&"nppdvjthqldpwncqszvftbrmjlhg")?, 23);
        assert_eq!(parse(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
        assert_eq!(parse(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);

        Ok(())
    }
}
