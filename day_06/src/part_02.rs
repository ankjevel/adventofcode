use std::collections::HashMap;

type Map = HashMap<String, String>;

fn get_val(input: &str, map: &Map) -> String {
    map.get(input).unwrap().to_owned()
}

pub fn main(input: &Vec<(String, String)>) -> u32 {
    let map = input.clone().into_iter().collect::<Map>();

    let orbit_list = |from: String| -> Vec<String> {
        let mut list = vec![from.clone()];
        let mut orbit = from.as_str();
        while map.contains_key(orbit) {
            orbit = map.get(orbit).unwrap();
            list.push(orbit.to_owned());
        }
        list
    };

    let orbits1 = orbit_list(get_val("YOU", &map));
    let orbits2 = orbit_list(get_val("SAN", &map));

    for (i, orbit1) in orbits1.iter().enumerate() {
        for (j, orbit2) in orbits2.iter().enumerate() {
            if orbit1 == orbit2 {
                return (i + j) as u32;
            }
        }
    }

    1
}
