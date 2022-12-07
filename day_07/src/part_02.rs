use std::io::Result;

use crate::{get_dirs, Input};

pub fn main(input: &Input) -> Result<u32> {
    let dirs = get_dirs(&input);
    let to_remove = 30_000_000 - (70_000_000 - dirs.get("/").unwrap_or(&0));

    Ok(dirs
        .into_values()
        .filter(|val| val >= &to_remove)
        .min()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 24933642);
        Ok(())
    }
}
