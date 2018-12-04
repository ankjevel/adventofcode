use std::io;

use claim::Claim;

pub fn main(claims: &Vec<Claim>) -> io::Result<u32> {
    let mut non_overlapping = 0;

    'main_loop: for a in claims.iter() {
        for b in claims.iter() {
            if a.id == b.id {
                continue;
            }

            if a.overlaps(b) {
                continue 'main_loop;
            }
        }

        non_overlapping = a.id;

        break 'main_loop;
    }

    Ok(non_overlapping)
}
