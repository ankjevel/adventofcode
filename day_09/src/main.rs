use ::day_09::program::exec;
use day_05::parse_input;

fn main() {
    let input = parse_input(include_str!("../../input/day_09"));
    println!("part_01, {:?}", exec(input[0].clone(), Some(vec![1])));
    println!("part_02, {:?}", exec(input[0].clone(), Some(vec![2])));
}

#[cfg(test)]
mod tests {
    use ::day_09::program::{exec, Program};
    use day_05::parse_input;
    use std::{sync::mpsc::channel, thread::spawn};

    const EXAMPLES_ON_PART_01: &'static str = "
        1102,34915192,34915192,7,4,7,99,0
        104,1125899906842624,99
    ";

    #[test]
    fn it_gets_the_examples_right_on_part_01() {
        let input = parse_input(&EXAMPLES_ON_PART_01);

        assert_eq!(exec(input[0].clone(), None), 1219070632396864);
        assert_eq!(exec(input[1].clone(), None), 1125899906842624);
    }

    #[test]
    fn quine() {
        let (b_sender, a_receiver) = channel();
        let (_, b_receiver) = channel();

        let output_thread = spawn(move || {
            let mut output = vec![];
            loop {
                let new_output = match a_receiver.recv() {
                    Ok(val) => val,
                    _ => break,
                };
                &output.push(new_output);
            }
            output.to_owned()
        });

        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let thread_input = input.clone();
        let result = spawn(move || {
            let mut program = Program::new(&thread_input);
            program.new_input(b_receiver);
            program.new_output(b_sender);
            program.run()
        })
        .join()
        .unwrap();

        assert_eq!(result, 99);

        assert_eq!(output_thread.join().unwrap(), input.clone());
    }
}
