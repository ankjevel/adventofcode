use std::str::from_utf8;

fn parse_input(string: &str) -> Vec<String> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .collect()
}

fn chars(input: &str, check: &char) -> Vec<char> {
    input
        .clone()
        .chars()
        .into_iter()
        .filter(|c| c == check)
        .into_iter()
        .collect()
}

fn len(input: &str, check: &char) -> usize {
    input.chars().into_iter().filter(|c| c == check).count()
}

fn chunk_str(input: &str, chunks: usize) -> Vec<String> {
    input
        .as_bytes()
        .chunks(chunks)
        .into_iter()
        .map(|c| from_utf8(&c).unwrap().to_owned())
        .collect::<Vec<_>>()
}

fn part_01(input: &str, chunks: usize) -> usize {
    let chunked = chunk_str(&input, chunks);
    let least_zeroes = chunked
        .clone()
        .into_iter()
        .fold(chunked[0].clone(), |acc, current| {
            let check = '0';
            if len(&current, &check) < len(&acc, &check) {
                current.to_owned()
            } else {
                acc
            }
        });

    chars(&least_zeroes, &'1').len() * chars(&least_zeroes, &'2').len()
}

fn part_02(input: &str, width: usize, height: usize) -> String {
    let chunk_size = width * height;
    let mut output = vec![b'2'; chunk_size];

    for chunk in input.as_bytes().chunks(chunk_size) {
        for (image_pixel, &layer_pixel) in output.iter_mut().zip(chunk.iter()) {
            if *image_pixel != b'2' {
                continue;
            }

            *image_pixel = layer_pixel;
        }
    }

    chunk_str(&from_utf8(&output)
        .unwrap()
        .replace('1', "*")
        .replace('0', " "), width).join("\n")
}

fn main() {
    let input = parse_input(include_str!("../../input/day_08"));

    println!("part_01: {:?}", part_01(&input[0], 25 * 6));
    println!("part_02:\n{}", part_02(&input[0], 25, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_01: &'static str = "01110222012102121112";

    const EXAMPLE_PART_02: &'static str = "0222112222120000";

    #[test]
    fn it_gets_the_first_example_right() {
        assert_eq!(part_01(&parse_input(&EXAMPLE_PART_01)[0], 4), 3);
    }

    #[test]
    fn it_get_the_second_example_right() {
        assert_eq!(
            part_02(&parse_input(&EXAMPLE_PART_02)[0], 2, 2),
            " *\n* "
        )
    }
}
