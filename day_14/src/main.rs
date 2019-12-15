use ::day_14::{parse_input, to_map, Item, Output};
use std::{collections::HashMap, mem::replace};

fn get_requirements(reactions: &HashMap<String, Output>, quantity: i64) -> i64 {
    let mut reserves: HashMap<String, i64> = HashMap::new();
    let mut required = HashMap::new();
    required.insert("FUEL".to_owned(), quantity.to_owned());

    while required.len() != 1 || !required.contains_key("ORE") {
        let mut new_rec = HashMap::new();

        for (chemical, quantity) in required.clone() {
            if chemical == "ORE" {
                *new_rec.entry("ORE".to_owned()).or_insert(0) += quantity.to_owned();
                continue;
            }

            let quantity = quantity as f64;
            let reaction = reactions.get(&chemical).unwrap();
            let reaction_quantity = reaction.quantity.to_owned() as f64;
            let reserve_ref = reserves.entry(chemical.to_owned()).or_insert(0);
            let reserve = reserve_ref.to_owned() as f64;
            let reaction_count = (quantity - reserve) / reaction_quantity;
            let reaction_count = reaction_count.ceil() as f64;

            for (ingredient, amount) in &reaction.required {
                let ingredient_ref = new_rec.entry(ingredient.to_owned()).or_insert(0);
                let ingredient_pre = ingredient_ref.to_owned() as f64;
                let amount = amount.to_owned() as f64;

                *ingredient_ref = (ingredient_pre + reaction_count * amount) as i64;
            }

            *reserve_ref = (reserve + reaction_count * reaction_quantity - quantity).ceil() as i64;
        }

        required = replace(&mut new_rec, HashMap::new());
    }

    required.entry("ORE".to_owned()).or_insert(0).to_owned()
}

type Input = Vec<(Item, Vec<Item>)>;

fn part_01(input: &Input) -> i64 {
    get_requirements(&to_map(input), 1)
}

fn part_02(input: &Input) -> i64 {
    let map = to_map(input);
    let total_ore: i64 = 1_000_000_000_000;
    let mut new_estimate = ((total_ore as f64) / (get_requirements(&map, 1) as f64)).floor() as i64;
    let mut estimate = 0;
    while new_estimate > estimate {
        estimate = new_estimate.to_owned();
        let needed = get_requirements(&map, estimate.to_owned());
        new_estimate = ((estimate as f64) * (total_ore as f64) / (needed as f64)).floor() as i64;
    }

    estimate
}

fn main() {
    let input = parse_input(include_str!("../../input/day_14"));
    println!("part_01: {}", part_01(&input));
    println!("part_02: {:?}", part_02(&input));
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;

    const EXAMPLE_01: &'static str = "
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    ";

    const EXAMPLE_02: &'static str = "
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    ";

    const EXAMPLE_03: &'static str = "
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF
    ";

    const EXAMPLE_04: &'static str = "
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
    ";

    #[test]
    fn it_gets_same_same_results_as_the_first_examples() {
        assert_eq!(part_01(&parse_input(&EXAMPLE_01)), 165);
        assert_eq!(part_01(&parse_input(&EXAMPLE_02)), 13312);
        assert_eq!(part_01(&parse_input(&EXAMPLE_03)), 180697);
        assert_eq!(part_01(&parse_input(&EXAMPLE_04)), 2210736);
    }

    #[test]
    fn it_gets_same_same_results_as_the_second_part() {
        assert_eq!(part_02(&parse_input(&EXAMPLE_02)), 82892753);
        assert_eq!(part_02(&parse_input(&EXAMPLE_03)), 5586022);
        assert_eq!(part_02(&parse_input(&EXAMPLE_04)), 460664);
    }
}
