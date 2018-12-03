extern crate regex;

mod claim;
mod part_01;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use regex::Regex;

use claim::Claim;


fn to_claims (string: String) -> io::Result<Vec<Claim>> {
    let mut claims = vec![];

    let re = Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();

    for line in string.lines() {
        if !re.is_match(&line) {
            continue
        }

        claims.push(Claim::new(line.to_string()))
    }

    Ok(claims)
}

pub fn input () -> io::Result<Vec<Claim>> {
    let input_file = try!(File::open("../input/day_03"));
    let mut file = BufReader::new(&input_file);
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let claims = to_claims(contents).unwrap();

    Ok(claims)
}

fn main() -> io::Result<()> {
    let claims = input().unwrap();
    println!(
        "How many square inches of fabric are within two or more claims? {}",
        part_01::main(&claims).unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_expected_claims() {
        let input = to_claims("broekn\n#1 @ 1,3: 4x4".to_string()).unwrap();
        let claim = input.first().unwrap();

        assert_eq!(claim.id, 1);
        assert_eq!(claim.rectangle.width, 4);
        assert_eq!(claim.rectangle.height, 4);
    }
}
