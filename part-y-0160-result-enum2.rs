fn main() {

    let age = "99";

    let age_num= age.parse::<u8>();

    println!("Age data: {:?}", age_num);

    let age = "99 years old";

    let age_num= age.parse::<u8>();

    println!("Age data: {:?}", age_num);
}

// Age data: Ok(99)
// Age data: Err(ParseIntError { kind: InvalidDigit })
