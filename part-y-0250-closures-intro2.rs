fn main() {

    let mult_nums = |x: i128, y: i128, z: i128| x * y * z;

    let a: u8 = 10; 

    let b = -5_000;

    let c = 42_754_000;

    println!("{a} * {b} * {c} = {}", mult_nums(a as i128, b, c));
}

// 10 * -5000 * 42754000 = -2137700000000
