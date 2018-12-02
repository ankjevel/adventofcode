use std::io;

mod part_01;
mod part_02;

fn main() -> io::Result<()> {
    println!("part 1; total: {}", part_01::main().unwrap());
    println!("part 2; reached_twice: {}", part_02::main().unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_total_right() {
        let result = part_01::main().unwrap();

        assert_eq!(result, 477)
    }

    #[test]
    fn get_value_first_reached_twice() {
        let result = part_02::main().unwrap();

        assert_eq!(result, 390)
    }
}
