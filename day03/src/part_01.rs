use std::io;

use claim::Claim;

pub fn main(claims: &Vec<Claim>) -> io::Result<i32> {
  // let mut grid: Vec<Vec<char>> = vec![];

  for claim in claims.iter() {
    println!(
      "id: {}, width: {}, height: {}, top_left.x: {}, top_left.y: {}, bottom_right.x: {}, bottom_right.y: {}",
      claim.id,
      claim.rectangle.width,
      claim.rectangle.height,
      claim.rectangle.top_left.x,
      claim.rectangle.top_left.y,
      claim.rectangle.bottom_right.x,
      claim.rectangle.bottom_right.y,
    );
  }

  Ok(0)
}
