use ::day_10::{parse_input, part_01::main as part_01, part_02::main as part_02, to_points};

fn main() {
    let input = to_points(&parse_input(include_str!("../../input/day_10")));
    let (result_part_01, input_for_part_02) = part_01(&input);
    println!("part_01: {}", result_part_01);
    println!("part_02: {}", part_02(&input, &input_for_part_02));
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use ::day_10::{
        parse_input, part_01::main as part_01, part_02::main as part_02, point::Point, to_points,
    };

    const EXAMPLE_01_00: &'static str = "
        .#..#
        .....
        #####
        ....#
        ...x#
    ";

    const EXAMPLE_01_01: &'static str = "
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...x..#.
        .#....####
    ";

    const EXAMPLE_01_02: &'static str = "
        #.#...#.#.
        .###....#.
        .x....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.
    ";

    const EXAMPLE_01_03: &'static str = "
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.x#.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..
    ";

    const EXAMPLE_01_04: &'static str = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.####x#####...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##
    ";

    #[test]
    fn it_gets_the_examples_on_part_01_right() {
        let inputs: Vec<Vec<Point>> = vec![
            EXAMPLE_01_00,
            EXAMPLE_01_01,
            EXAMPLE_01_02,
            EXAMPLE_01_03,
            EXAMPLE_01_04,
        ]
        .into_iter()
        .map(|example| to_points(&parse_input(&example)))
        .collect();

        assert_eq!(part_01(&inputs[0]), (8, Some(Point { x: 3, y: 4 })));
        assert_eq!(part_01(&inputs[1]), (33, Some(Point { x: 5, y: 8 })));
        assert_eq!(part_01(&inputs[2]), (35, Some(Point { x: 1, y: 2 })));
        assert_eq!(part_01(&inputs[3]), (41, Some(Point { x: 6, y: 3 })));
        assert_eq!(part_01(&inputs[4]), (210, Some(Point { x: 11, y: 13 })));
    }

    #[test]
    fn it_gets_the_example_on_part_02_right() {
        let input = to_points(&parse_input(&EXAMPLE_01_04));
        let start = Some(Point { x: 11, y: 13 });

        assert_eq!(part_02(&input, &start), 802);
    }
}
