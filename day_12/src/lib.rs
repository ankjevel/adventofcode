extern crate num_integer;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Moon {
    pub position: (i64, i64, i64),
    pub velocity: (i64, i64, i64),
}

impl Moon {
    pub fn new(input: Vec<i64>) -> Moon {
        let mut iter = input.into_iter();

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();

        Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        }
    }

    pub fn total_energy(&self) -> i64 {
        let (x, y, z) = self.position;
        let (dx, dy, dz) = self.velocity;

        let pot = x.abs() + y.abs() + z.abs();
        let kin = dx.abs() + dy.abs() + dz.abs();

        pot * kin
    }

    pub fn gravity(&self, compared: &Moon) -> (i64, i64, i64) {
        let (ax, ay, az) = self.position;
        let (bx, by, bz) = compared.position;

        (gravity(ax, bx), gravity(ay, by), gravity(az, bz))
    }
}

pub fn gravity(a: i64, b: i64) -> i64 {
    if a > b {
        -1
    } else if b > a {
        1
    } else {
        0
    }
}

pub fn new_moon(moon: &Moon, energy: &(i64, i64, i64)) -> Moon {
    let (ddx, ddy, ddz) = energy;

    let (dx, dy, dz) = moon.velocity;
    let (dx, dy, dz) = (dx + ddx, dy + ddy, dz + ddz);

    let (x, y, z) = moon.position;
    let (x, y, z) = (x + dx, y + dy, z + dz);

    Moon {
        position: (x, y, z),
        velocity: (dx, dy, dz),
    }
}

pub fn parse_input(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|part| {
            part.split(',')
                .map(str::trim)
                .map(str::to_owned)
                .map(|s| {
                    s.replace('>', "")
                        .replace('<', "")
                        .split('=')
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()
                })
                .collect()
        })
        .map(Moon::new)
        .collect()
}
