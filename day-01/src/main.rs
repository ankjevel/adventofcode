use std::io;

mod part_01;
mod part_02;

fn main() -> io::Result<()> {
    println!("part 1; total: {}", part_01::main().unwrap());
    println!("part 2; reaced_twice: {}", part_02::main().unwrap());

    Ok(())
}
