use std::ops::Add;

fn main() {

    let _sum1 = sum_vals(3.123f32, 43.555);

    let _sum2 = sum_vals(-21i16, 92);
}

fn sum_vals<T>(x: T, y: T) -> T 

where 
    T: Add<Output =  T>,
{
    x + y
}
