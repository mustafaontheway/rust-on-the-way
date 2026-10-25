fn main() {

    let mut ages = vec![22u8, 14, 99, 75, 18, 23];

    for age in &mut ages {

        *age -= 2;
    }

    println!("{:?}", ages) // [20, 12, 97, 73, 16, 21]
}


