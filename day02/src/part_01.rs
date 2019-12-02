use std::io;

pub fn main(input: &Vec<u32>) -> io::Result<Vec<u32>> {
    let mut copy_of_input = input.clone();
    let mut current_position = 0;
    loop {
        let opcode = copy_of_input[current_position];
        if opcode == 99 {
            break;
        }

        let noun = copy_of_input[copy_of_input[current_position + 1] as usize];
        let verb = copy_of_input[copy_of_input[current_position + 2] as usize];
        let output_position = copy_of_input[current_position + 3] as usize;
        copy_of_input[output_position] = match opcode {
            1 => noun + verb,
            2 => noun * verb,
            _ => panic!("unexpected opcode: {}", opcode),
        };
        current_position += 4;
    }
    Ok(copy_of_input)
}
