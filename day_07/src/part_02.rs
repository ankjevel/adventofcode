use std::io::Result;

use crate::Input;

fn lookup((size, name): &(u8, String), input: &Input) -> usize {
    let size = size.to_owned() as usize;
    size + size * look_inside(input.get(name).unwrap(), input)
}

fn look_inside(inside: &Vec<(u8, String)>, input: &Input) -> usize {
    inside.iter().map(|item| lookup(item, input)).sum()
}

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .get("shiny gold")
        .unwrap()
        .iter()
        .map(|item| lookup(item, input))
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_first_example_correct() -> Result<()> {
        assert_eq!(
            main(&parse_input(
                &"  light red bags contain 1 bright white bag, 2 muted yellow bags.
                    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                    bright white bags contain 1 shiny gold bag.
                    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                    faded blue bags contain no other bags.
                    dotted black bags contain no other bags."
            ))?,
            32
        );
        Ok(())
    }

    #[test]
    fn it_gets_second_example_correct() -> Result<()> {
        assert_eq!(
            main(&parse_input(
                &"  shiny gold bags contain 2 dark red bags.     
                    dark red bags contain 2 dark orange bags.    
                    dark orange bags contain 2 dark yellow bags. 
                    dark yellow bags contain 2 dark green bags.  
                    dark green bags contain 2 dark blue bags.    
                    dark blue bags contain 2 dark violet bags.   
                    dark violet bags contain no other bags."
            ))?,
            126
        );
        Ok(())
    }
}
