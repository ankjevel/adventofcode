use crate::Input;

pub fn evaluate(input: &Input, iterations: i32, division: i64) -> i64 {
    let mut input = input.to_owned();
    let modulus = input
        .values()
        .map(|monkey| monkey.divisible_by)
        .product::<i64>();

    for _ in 0..iterations {
        let mut index = 0;
        'main: loop {
            let mut throw_to_monkeys: Vec<_> = vec![];
            {
                if let Some(monkey) = input.get_mut(&index) {
                    monkey.inspected += monkey.items.len() as i64;
                    'inner: loop {
                        if let Some(item) = monkey.items.pop_front() {
                            let worry_level =
                                (monkey.handle_item(&item.to_owned()) % modulus) / division;
                            let throw_to = monkey.throw_to(worry_level);
                            throw_to_monkeys.push((throw_to, worry_level));
                        } else {
                            break 'inner;
                        }
                    }
                }
            }

            {
                for (index, worry_level) in throw_to_monkeys.clone() {
                    if let Some(monkey) = input.get_mut(&index) {
                        monkey.items.push_back(worry_level);
                    }
                }
            }

            index += 1;

            if index >= input.len() as isize {
                break 'main;
            }
        }
    }

    let mut values = input
        .to_owned()
        .values()
        .map(|m| m.inspected)
        .collect::<Vec<_>>();

    values.sort();
    values.reverse();

    values.get(0).unwrap() * values.get(1).unwrap()
}
