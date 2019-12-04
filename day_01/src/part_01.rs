use std::io;

pub fn main(input: &Vec<i32>) -> io::Result<Vec<i32>> {
    Ok(input.iter()
        .map(|&mass| ((mass.clone() as f64 / 3.0).floor() as i32) - 2)
        .collect())
}
