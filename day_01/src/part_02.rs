use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

use regex::Regex;

type Change = (char, i32);

fn get_freqencies() -> io::Result<Vec<Change>> {
    let input_file = File::open("../input/day_01")?;
    let file = io::BufReader::new(&input_file);

    let mut freq: Vec<Change> = vec![];
    let re = Regex::new(r"[+-]\d").unwrap();

    for line in file.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();

        if !re.is_match(&String::from_iter(chars.iter())) {
            continue;
        }

        let n_str: String = chars[1..chars.len()].iter().collect();
        let n: i32 = n_str.parse::<i32>().unwrap();

        freq.push((chars[0], n));
    }

    Ok(freq)
}

pub fn main() -> io::Result<i32> {
    let input = get_freqencies()?;

    let mut reaced_twice: i32 = 0;

    let mut used_frequencies = HashSet::new();
    let mut frequency = 0;

    'outer_loop: loop {
        for (modulus, n) in input.iter() {
            match modulus.to_string().as_str() {
                "+" => frequency += n,
                "-" => frequency -= n,
                _ => break 'outer_loop,
            }

            if !used_frequencies.contains(&frequency) {
                used_frequencies.insert(frequency.clone());
                continue;
            }

            reaced_twice = frequency;
            break 'outer_loop;
        }
    }

    Ok(reaced_twice)
}
