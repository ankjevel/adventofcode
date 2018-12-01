use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn main() -> io::Result<i32> {
    let input_file = try!(File::open("src/input"));
    let file = io::BufReader::new(&input_file);
    let mut total = 0;

    for line in file.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();

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
