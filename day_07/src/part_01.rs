use std::{
    collections::{HashMap, VecDeque},
    io::Result,
};

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let mut path: VecDeque<String> = VecDeque::new();
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for row in input.into_iter() {
        if row.starts_with('$') {
            if row.contains("$ cd") {
                let next_dir = row.replace("$ cd ", "");
                if next_dir.contains("..") {
                    path.pop_back();
                } else {
                    path.push_back(if next_dir.contains('/') {
                        next_dir
                    } else {
                        format!("{}/", next_dir)
                    });
                }
            }
        } else {
            if !row.starts_with("dir ") {
                let n: Vec<&str> = row.split(' ').collect();
                let size = n[0].parse::<u32>().unwrap();
                let mut current: Vec<String> = vec![];
                for path in path.clone().into_iter() {
                    current.push(path);
                    let path = current.join("");
                    *dirs.entry(path).or_insert(0) += size;
                }
            }
        }
    }

    Ok(dirs
        .into_values()
        .filter(|val| val <= &100_000)
        .sum::<u32>())
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 95437);
        Ok(())
    }
}
