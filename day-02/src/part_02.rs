use std::io;
use std::iter::FromIterator;

type Diff = (Vec<char>, i32);

fn compare_chars (a: Vec<char>, b: Vec<char>) -> Diff {
    let mut chars = vec![];
    let mut differ = 0;

    for (a, b) in a.iter().zip(b.iter()) {
        if a == b {
            chars.push(*a)
        } else {
            differ += 1
        }
    }

    (chars, differ)
}

pub fn main(rows: Vec<Vec<char>>) -> io::Result<String> {
    'main_loop: for a in rows.iter() {
        for b in rows.iter() {
            let (chars, differ) = compare_chars(a.to_vec(), b.to_vec());
            if differ == 1 {
                return Ok(String::from_iter(chars));
            }
        }
    }

    Ok("".to_string())
}
