use std::io::Result;

use crate::Input;

pub fn main((stacks, commands): &Input) -> Result<String> {
    let mut stacks = stacks.to_owned();

    for (n, from, to) in commands.to_owned() {
        let from_stack = stacks.get_mut(&from).unwrap();
        let mut new_vals: Vec<char> = vec![];
        for _ in 1..=n {
            let val = from_stack.pop_front().unwrap();
            new_vals.push(val);
        }

        new_vals.reverse();

        new_vals
            .iter()
            .for_each(|val| stacks.get_mut(&to).unwrap().push_front(val.to_owned()));
    }

    Ok(stacks
        .into_iter()
        .map(|(_, stack)| stack.front().unwrap().to_owned())
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, "MCD");
        Ok(())
    }
}
