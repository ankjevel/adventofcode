use std::collections::HashMap;

type Map = HashMap<String, String>;

pub fn main(input: &Vec<(String, String)>) -> u32 {
    let map = input.clone().into_iter().collect::<Map>();
    let mut results = 0;

    'foreach: for key in map.keys() {
        let mut current = key;
        'search: loop {
            match map.get(current) {
                Some(x) => {
                    results += 1;
                    current = x;
                }
                None => break 'search,
            };
        }
    }

    results
}
