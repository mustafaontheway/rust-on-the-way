use std::ops::Div;

use num_traits::Zero;

fn main() {

    let div1 = secure_div(12, 77);

    println!("{div1:?}");

    let div2 = secure_div(12.999, 0.0);

    println!("{div2:?}");
}

fn secure_div<T>(x: T, y: T) -> Result<T, String> 
where 
    T: Div<Output = T> + PartialEq + Zero
{

    if !y.is_zero() {

        Ok(x / y)

    } else {

        Err("Zero devision error!".to_string())
    }
}
