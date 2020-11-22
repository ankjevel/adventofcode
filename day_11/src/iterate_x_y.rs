pub fn iterate_x_y<F>(min: u16, max: u16, mut call: F)
where
    F: FnMut(u16, u16) -> (),
{
    iterate_square(min, max, min, max, &mut call)
}

fn iterate_square<F>(x_min: u16, x_max: u16, y_min: u16, y_max: u16, mut call: F)
where
    F: FnMut(u16, u16) -> (),
{
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            call(x, y)
        }
    }
}
