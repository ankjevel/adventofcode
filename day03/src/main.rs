extern crate regex;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use regex::Regex;

#[derive(Debug)]
pub struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

pub fn input () -> io::Result<Vec<Claim>> {
    let input_file = try!(File::open("../input/day_03"));
    let file = io::BufReader::new(&input_file);

    let mut rows = vec![];
    let re = Regex::new(r"^#(\d)* @ \d*,\d*: \d*x\d*$").unwrap();

    for line in file.lines() {
        let r = line.unwrap();

        if !re.is_match(&r) {
            continue
        }
    }

    Ok(rows)
}

fn main() -> io::Result<()> {
    let rows = input().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
