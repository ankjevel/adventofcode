use std::io;
use std::io::prelude::*;
use std::fs::File;

mod part_01;
mod part_02;

fn input () -> io::Result<Vec<Vec<char>>> {
    let input_file = try!(File::open("../input/day_02"));
    let file = io::BufReader::new(&input_file);

    let mut rows = vec![];
    for line in file.lines() {
        let r = line.unwrap();
        let chars: Vec<char> = r.chars().collect();

        rows.push(chars);
    }

    Ok(rows)
}

fn main() -> io::Result<()> {
    println!("part 1; checksum: {}", part_01::main(input().unwrap()).unwrap());
    println!("part 2; common letters: {}", part_02::main(input().unwrap()).unwrap());

    Ok(())
}


#[test]
fn it_gets_the_expected_checksum_from_example() {
    let input = vec![
        vec!['a', 'b', 'c', 'd', 'e', 'f'],
        vec!['b', 'a', 'b', 'a', 'b', 'c'],
        vec!['a', 'b', 'b', 'c', 'd', 'e'],
        vec!['a', 'b', 'c', 'c', 'c', 'd'],
        vec!['a', 'a', 'b', 'c', 'd', 'd'],
        vec!['a', 'b', 'c', 'd', 'e', 'e'],
        vec!['a', 'b', 'a', 'b', 'a', 'b'],
    ];

    let result = part_01::main(input).unwrap();

    assert_eq!(result, 12)
}

#[test]
fn gets_the_actual_checksum_right() {
    let rows = input().unwrap();
    let result = part_01::main(rows).unwrap();

    assert_eq!(result, 4920)
}

#[test]
fn it_gets_the_common_letters_from_example() {
    let input = vec![
        vec!['a', 'b', 'c', 'd', 'e'],
        vec!['f', 'g', 'h', 'i', 'j'],
        vec!['k', 'l', 'm', 'n', 'o'],
        vec!['p', 'q', 'r', 's', 't'],
        vec!['f', 'g', 'u', 'i', 'j'],
        vec!['a', 'x', 'c', 'y', 'e'],
        vec!['w', 'v', 'x', 'y', 'z'],
    ];

    let result = part_02::main(input).unwrap();

    assert_eq!(result, "fgij".to_string())
}

#[test]
fn gets_the_actual_common_letters_right() {
    let rows = input().unwrap();
    let result = part_02::main(rows).unwrap();

    assert_eq!(result, "fonbwmjquwtapeyzikghtvdxl".to_string())
}
