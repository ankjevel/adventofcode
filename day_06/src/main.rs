mod part_01;
mod part_02;

use part_01::main as part_01;
use part_02::main as part_02;

fn main() {
    let input = parse_input(include_str!("../../input/day_06"));
    println!("part_01: {}", part_01(&input));
    println!("part_02: {}", part_02(&input));
}

fn parse_input(string: &str) -> Vec<(String, String)> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|line| line.split(')').map(str::to_owned).collect())
        .map(|x: Vec<String>| (x[1].clone(), x[0].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_01::main as part_01;
    use super::part_02::main as part_02;

    const EXAMPLE_DATA_PART_01: &'static str = "
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
    ";

    const UNORDERED_EXAMPLE_DATA_PART_01: &'static str = "
        K)L
        E)J
        J)K
        COM)B
        E)F
        B)C
        D)I
        B)G
        G)H
        D)E
        C)D
    ";

    const EXAMPLE_DATA_PART_02: &'static str = "
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN
    ";

    const UNORDERED_EXAMPLE_DATA_PART_02: &'static str = "
        B)G
        J)K
        E)F
        B)C
        COM)B
        G)H
        I)SAN
        D)E
        C)D
        E)J
        K)L
        K)YOU
        D)I
    ";

    #[test]
    fn it_gets_the_example_right_for_part_01() {
        let example_data = parse_input(&EXAMPLE_DATA_PART_01);
        let unordered_example_data = parse_input(&UNORDERED_EXAMPLE_DATA_PART_01);

        assert_eq!(part_01(&example_data), 42);
        assert_eq!(part_01(&unordered_example_data), 42);
    }

    #[test]
    fn it_gets_the_example_right_for_part_02() {
        let example_data = parse_input(&EXAMPLE_DATA_PART_02);
        let unordered_example_data = parse_input(&UNORDERED_EXAMPLE_DATA_PART_02);

        assert_eq!(part_02(&example_data), 4);
        assert_eq!(part_02(&unordered_example_data), 4);
    }

}
