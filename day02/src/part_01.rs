use std::io;

pub fn main(input: &Vec<u32>) -> io::Result<Vec<u32>> {
    let mut copy_of_input = input.clone();
    let mut i = 0;

    loop {
        let opcode = copy_of_input[i];

        if opcode == 99 {
            break;
        }

        let x = copy_of_input[copy_of_input[i + 1] as usize];
        let y = copy_of_input[copy_of_input[i + 2] as usize];
        let result = copy_of_input[i + 3] as usize;

        copy_of_input[result] = match opcode {
            1 => x + y,
            2 => x * y,
            _ => panic!("unexpected opcode: {}", opcode),
        };

        i = i + 4;
    }

    Ok(copy_of_input)
}
