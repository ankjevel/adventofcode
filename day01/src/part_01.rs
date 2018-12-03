use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

use regex::Regex;

pub fn main() -> io::Result<i32> {
    let input_file = try!(File::open("../input/day_01"));
    let file = io::BufReader::new(&input_file);
    let mut total = 0;

    let re = Regex::new(r"[+-]\d").unwrap();

    for line in file.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();

        if !re.is_match(&String::from_iter(chars.iter())) {
            continue;
        }

        let n_str: String = chars[1..chars.len()].iter().collect();
        let n: i32 = n_str.parse::<i32>().unwrap();

        match chars[0].to_string().as_str() {
            "+" => total += n,
            "-" => total -= n,
            _ => {}
        }
    }

    Ok(total)
}
