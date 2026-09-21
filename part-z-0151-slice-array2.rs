fn main() {

    let ages = [17u8, 99, 21, 87, 65, 40, 33];

    let _s1 = &ages[..3];

    let _s2 = &ages[..=3];

    let _s3 = &ages[..];

    print_array_slice(_s3);
}

fn print_array_slice(s: &[u8]) {

    println!("Array slice: {s:?}")
}
