use std::ops::{Add, Mul};

fn main() {

    let (_sum, _mult) = sum_or_mult(4.21f32, 3.18);

    let (_sum, _mult) = sum_or_mult(47i32, -500);
}

fn sum_or_mult<T>(x: T, y: T) -> (T, T)

where 
    T: Add<Output = T>,
    T: Mul<Output = T>,
    T: Copy
{
    (x + y, x * y)
}

