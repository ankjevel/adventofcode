use std::io;
use std::collections::HashMap;

use claim::Claim;

pub fn main(claims: &Vec<Claim>) -> io::Result<i32> {
    let mut fabric = HashMap::new();

    for x in 0..1000 {
        for y in 0..1000 {
            for claim in claims.iter() {
                if claim.contains(x, y) {
                    fabric.entry((x, y)).and_modify(|fabric| *fabric += 1).or_insert(1);
                }
            }
        }
    }

    let overlaps = fabric
        .iter()
        .filter(|&(_, num)| *num > 1)
        .map(|(_, num)| *num)
        .collect::<Vec<i32>>()
        .len() as i32;

    Ok(overlaps)
}
