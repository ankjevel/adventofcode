use std::{
    mem,
    ops::{Div, Mul, Rem, Sub},
};

pub fn lcm<
    T: Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Rem<Output = T> + Ord + Eq + Copy,
>(
    a: T,
    b: T,
) -> T {
    a * b / gcd(a, b)
}

pub fn gcd<
    T: Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Rem<Output = T> + Ord + Eq + Copy,
>(
    a: T,
    b: T,
) -> T {
    let mut max = a;
    let mut min = b;

    if min > max {
        mem::swap(&mut max, &mut min);
    }

    loop {
        let result = max % min;
        let zero = result - result;
        if result == zero {
            return min;
        }

        max = min;
        min = result;
    }
}
