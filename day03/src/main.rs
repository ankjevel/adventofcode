extern crate colored;
extern crate regex;

mod claim;
mod part_01;
mod part_02;
#[allow(dead_code)]
mod print_grid;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use claim::Claim;
use regex::Regex;

fn main() -> io::Result<()> {
    let claims = input().unwrap();

    println!(
        "How many square inches of fabric are within two or more claims? {}",
        part_01::main(&claims).unwrap()
    );

    println!(
        "What is the ID of the only claim that doesn't overlap? {}",
        part_02::main(&claims).unwrap()
    );

    // println!("{}", print_grid::main(&claims).unwrap());

    Ok(())
}

fn to_claims(string: String) -> io::Result<Vec<Claim>> {
    let re = Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();

    let claims = string
        .lines()
        .map(|line| line.trim())
        .filter(|line| re.is_match(&line))
        .map(|line| Claim::new(line.to_string()))
        .collect::<Vec<Claim>>();

    Ok(claims)
}

pub fn input() -> io::Result<Vec<Claim>> {
    let input_file = try!(File::open("../input/day_03"));
    let mut file = BufReader::new(&input_file);
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let claims = to_claims(contents).unwrap();

    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        #1 @ 1,3: 4x4
        #2 @ 3,1: 4x4
        #3 @ 5,5: 2x2
    ";

    fn get_claims() -> io::Result<Vec<Claim>> {
        let claims = to_claims(EXAMPLE_DATA.to_string()).unwrap();

        Ok(claims)
    }

    #[test]
    fn generate_expected_claims() {
        let claims = get_claims().unwrap();
        let claim = claims.first().unwrap();

        assert_eq!(claim.id, 1);
        assert_eq!(claim.rectangle.width, 4);
        assert_eq!(claim.rectangle.height, 4)
    }

    #[test]
    fn it_handles_test_data() {
        let claims = get_claims().unwrap();
        let result = part_01::main(&claims).unwrap();

        assert_eq!(result, 4)
    }

    #[test]
    fn return_the_none_overlapping_id() {
        let claims = get_claims().unwrap();
        let result = part_02::main(&claims).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn testing_printer() {
        let grid: String = "
            ........
            ...2222.
            ...2222.
            .11xx22.
            .11xx22.
            .111133.
            .111133.
            ........"
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n");

        let claims = get_claims().unwrap();

        assert_eq!(print_grid::main(&claims).unwrap(), grid)
    }
}
